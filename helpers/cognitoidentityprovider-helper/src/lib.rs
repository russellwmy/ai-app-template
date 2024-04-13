use aws_sdk_cognitoidentityprovider::{
    operation::admin_get_user::AdminGetUserOutput,
    types::{AuthFlowType, UserType},
    Client, Error,
};
use tracing::{error, info};

pub async fn get_user(
    client: &Client,
    user_pool_id: &str,
    username: &str,
) -> Result<AdminGetUserOutput, Error> {
    let get_output = client
        .admin_get_user()
        .user_pool_id(user_pool_id)
        .username(username)
        .send()
        .await?;

    Ok(get_output)
}

pub async fn get_user_by_email(
    client: &Client,
    user_pool_id: &str,
    email: &str,
) -> Result<Option<UserType>, Error> {
    let list_output = client
        .list_users()
        .limit(1)
        .user_pool_id(user_pool_id)
        .filter(format!("email=\"{}\"", email))
        .send()
        .await?;
    let users_output = list_output.users();
    match users_output.first() {
        Some(user) => Ok(Some(user.clone())),
        None => Ok(None),
    }
}
pub async fn create_anonymous_user(
    client: &Client,
    user_pool_id: &str,
    client_id: &str,
    email: &str,
) -> Result<serde_json::Value, Error> {
    let pg = passwords::PasswordGenerator::new()
        .length(12)
        .numbers(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .exclude_similar_characters(true)
        .strict(true);
    let passwords = pg.generate(1).unwrap();
    let password = passwords.first().unwrap();

    match client
        .sign_up()
        .client_id(client_id)
        .username(email)
        .password(password)
        .send()
        .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("{}", e.to_string());
            None
        }
    };
    info!("created anonymous");

    client
        .admin_add_user_to_group()
        .user_pool_id(user_pool_id)
        .username(email)
        .group_name("Anonymous")
        .send()
        .await?;
    info!("added anonymous user to group");

    client
        .admin_confirm_sign_up()
        .user_pool_id(user_pool_id)
        .username(email)
        .send()
        .await?;
    info!("confirmed anonymous user");

    let result = client
        .admin_initiate_auth()
        .user_pool_id(user_pool_id)
        .client_id(client_id)
        .auth_flow(AuthFlowType::AdminNoSrpAuth)
        .auth_parameters("USERNAME", email)
        .auth_parameters("PASSWORD", password)
        .send()
        .await?;
    let authentication_result = result.authentication_result().unwrap();

    let data = serde_json::json!({
        "email": email,
        "password": password,
        "refresh_token": authentication_result.refresh_token(),
        "access_token": authentication_result.access_token(),
        "id_token": authentication_result.id_token(),
    });
    Ok(data)
}

pub async fn delete_anonymous_user(
    client: &Client,
    user_pool_id: &str,
    email: &str,
) -> Result<(), Error> {
    let _ = client
        .admin_delete_user()
        .user_pool_id(user_pool_id)
        .username(email)
        .send()
        .await?;
    Ok(())
}
