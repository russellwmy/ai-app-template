use anyhow::Result;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, to_attribute_value, to_item};

use crate::Task;

fn get_table_name() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "app_message_queue".to_string(),
        _ => "app_message_queue_stage".to_string(),
    };
    Ok(value)
}

pub async fn get_task(client: &Client, id: &str) -> Result<Task> {
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

pub async fn put_task(client: &Client, data: Task) -> Result<()> {
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
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub async fn delete_task(client: &Client, id: &str) -> Result<()> {
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
