use anyhow::Result;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, to_attribute_value, to_item};
use tracing::info;

use super::models::Group;

fn get_table_name() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "app_group".to_string(),
        _ => "app_group_stage".to_string(),
    };
    Ok(value)
}

pub async fn get_group(client: &Client, id: &str) -> Result<Group> {
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

pub async fn put_group(client: &Client, data: Group) -> Result<()> {
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

pub async fn delete_group(client: &Client, id: &str) -> Result<()> {
    let table = get_table_name()?;
    match client
        .delete_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
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

pub async fn get_group_creator(client: &Client, creator_id: &str) -> Result<Group> {
    let table = get_table_name()?;
    match client
        .scan()
        .table_name(&table)
        .filter_expression("#key = :value")
        .expression_attribute_names("#key", "creator_id")
        .expression_attribute_values(":value", to_attribute_value(creator_id)?)
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

pub async fn check_group_member(client: &Client, group_id: &str, user_id: &str) -> Result<()> {
    let group = get_group(client, group_id).await?;
    match group.user_ids().contains(&user_id.to_string()) {
        true => Ok(()),
        false => Err(anyhow::anyhow!("not a member")),
    }
}

pub async fn add_user_to_group(client: &Client, id: &str, user_id: &str) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_users = list_append(#attr_users, :user_value)")
        .expression_attribute_names("#attr_users", "users")
        .expression_attribute_values(":user_value", to_attribute_value(vec![user_id])?);

    info!("Send update item request to dyanmodb {}", table);

    match request.send().await {
        Ok(_) => {
            info!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn remove_user_to_group(client: &Client, id: &str, user_id: &str) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_users = list_remove(:user_value)")
        .expression_attribute_names("#attr_users", "users")
        .expression_attribute_values(":user_value", to_attribute_value(user_id)?);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}
