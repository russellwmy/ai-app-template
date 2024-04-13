use common::extract_sub_from_jwt;
use database::{get_user, put_user, User};
use lambda_http::{Request, RequestExt};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Serialize, Deserialize)]
pub struct PutPayload {
    name: String,
}

enum PutRoutes {
    SyncCognito,
}

pub(crate) async fn process_put_request(
    request: Request,
    context: &Context,
) -> anyhow::Result<serde_json::Value> {
    let token = request.headers().get("Authorization").unwrap().to_str()?;
    let user_id = extract_sub_from_jwt(token)?;
    let mut router = Router::new();
    router.add("/users/cognito/sync", PutRoutes::SyncCognito);

    let routing = router.recognize(&request.raw_http_path());
    match routing {
        Ok(routing) => match routing.handler() {
            PutRoutes::SyncCognito => {
                let user_result = get_user(&context.dynamodb_client, &user_id).await;
                match user_result {
                    Ok(user) => Ok(serde_json::to_value(user)?),
                    Err(_) => {
                        let user_pool_id = common::vars::get_aws_cognito_user_pool_id()?;
                        let cognito_user = cognitoidentityprovider_helper::get_user(
                            &context.cognitoidentityprovider_client,
                            &user_pool_id,
                            &user_id,
                        )
                        .await?;

                        let email_attr = cognito_user
                            .user_attributes()
                            .into_iter()
                            .find(|a| a.name() == "email");

                        match email_attr {
                            Some(email_attr) => {
                                let user = User::builder()
                                    .id(user_id.to_owned())
                                    .email(email_attr.value().unwrap().to_owned())
                                    .build();
                                put_user(&context.dynamodb_client, user.to_owned()).await?;
                                Ok(serde_json::to_value(user)?)
                            }
                            None => Err(anyhow::anyhow!("fail to sync user")),
                        }
                    }
                }
            }
        },
        Err(_) => Err(anyhow::anyhow!("Not found")),
    }
}
