use crate::get_group;

use super::Document;
use anyhow::Result;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, to_attribute_value, to_item};

fn get_table_name() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "app_document".to_string(),
        _ => "app_document_stage".to_string(),
    };
    Ok(value)
}

pub async fn get_document(client: &Client, id: &str) -> Result<Document> {
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

pub async fn put_document(client: &Client, data: Document) -> Result<()> {
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

pub async fn delete_document(client: &Client, id: &str) -> Result<()> {
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

pub async fn get_documents_by_group_id(client: &Client, group_id: &str) -> Result<Vec<Document>> {
    let table = get_table_name()?;
    match client
        .scan()
        .table_name(&table)
        .filter_expression("#attr_group_id = :group_id_value")
        .expression_attribute_names("#attr_group_id", "group_id")
        .expression_attribute_values(":group_id_value", to_attribute_value(group_id)?)
        .send()
        .await
    {
        Ok(output) => {
            println!("Query item from table: {}", table);
            Ok(output
                .items()
                .iter()
                .filter_map(|item| from_item(item.clone()).ok())
                .collect::<Vec<Document>>())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn get_documents(client: &Client, document_ids: Vec<String>) -> Vec<Document> {
    let mut results = vec![];

    for document_id in document_ids {
        if let Ok(result) = get_document(client, &document_id).await {
            results.push(result);
        }
    }

    results
}

pub async fn update_document_status(client: &Client, id: &str, status: &str) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_status = :status_value")
        .expression_attribute_names("#attr_status", "status")
        .expression_attribute_values(":status_value", to_attribute_value(status)?);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn update_document_index_state(
    client: &Client,
    id: &str,
    index_state: &str,
) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_index_state = :index_state_value")
        .expression_attribute_names("#attr_index_state", "index_state")
        .expression_attribute_values(":index_state_value", to_attribute_value(index_state)?);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn check_document_permission(
    client: &Client,
    document: &Document,
    user_id: &str,
) -> Result<()> {
    if document.creator_id() == user_id {
        return Ok(());
    }

    let group = get_group(client, &document.group_id()).await?;
    if group.owner_ids().contains(&user_id.to_string()) {
        return Ok(());
    }

    Err(anyhow::anyhow!("Access Denied"))
}
