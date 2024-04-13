use anyhow::Result;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, to_attribute_value, to_item};

use super::User;

fn get_table_name() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "app_user".to_string(),
        _ => "app__user_stage".to_string(),
    };
    Ok(value)
}

pub async fn get_user(client: &Client, id: &str) -> Result<User> {
    let table = get_table_name()?;
    match client
        .query()
        .table_name(&table)
        .key_condition_expression("#key = :value")
        .expression_attribute_names("#key", "id")
        .expression_attribute_values(":value", to_attribute_value(id)?)
        .send()
        .await
    {
        Ok(output) => {
            println!("Query item from table: {}", table);
            match output.items().first() {
                Some(item) => Ok(from_item(item.to_owned())?),
                None => Err(anyhow::anyhow!("No record")),
            }
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn get_user_by_email(client: &Client, email: &str) -> Result<User> {
    let table = get_table_name()?;
    match client
        .query()
        .table_name(&table)
        .key_condition_expression("#key_email = :email_value")
        .expression_attribute_names("#key_email", "email")
        .expression_attribute_values(":email_value", to_attribute_value(email)?)
        .send()
        .await
    {
        Ok(output) => {
            println!("Query item from table: {}", table);
            match output.items().first() {
                Some(item) => Ok(from_item(item.to_owned())?),
                None => Err(anyhow::anyhow!("No record")),
            }
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn put_user(client: &Client, data: User) -> Result<()> {
    let table = get_table_name()?;
    let item = to_item(data)?;

    match client
        .put_item()
        .table_name(&table)
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => {
            println!("Query item from table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}
