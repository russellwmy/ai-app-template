use crate::get_group;

use super::Collection;
use anyhow::Result;
use aws_sdk_dynamodb::{types::ReturnValue, Client};
use serde_dynamo::{from_item, to_attribute_value, to_item};
use tracing::info;

fn get_table_name() -> Result<String, std::env::VarError> {
    let value = match common::vars::get_app_environment()? {
        common::Environment::Production => "app_collection".to_string(),
        _ => "app_collection_stage".to_string(),
    };
    Ok(value)
}

pub async fn get_collection(client: &Client, id: &str) -> Result<Collection> {
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

pub async fn put_collection(client: &Client, data: Collection) -> Result<()> {
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

pub async fn delete_collection(client: &Client, id: &str) -> Result<()> {
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

pub async fn get_collections_by_group_id(
    client: &Client,
    group_id: &str,
) -> Result<Vec<Collection>> {
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
                .collect::<Vec<Collection>>())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn update_collection_name(client: &Client, id: &str, name: &str) -> Result<()> {
    info!("called update_collection_name, id:{}, name: {}", id, name);
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_name = :name_value")
        .expression_attribute_names("#attr_name", "name")
        .expression_attribute_values(":name_value", to_attribute_value(name)?)
        .return_values(ReturnValue::AllNew);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn update_collection_documents(
    client: &Client,
    id: &str,
    documents: Vec<String>,
) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_document_ids = :document_ids_value")
        .expression_attribute_names("#attr_document_ids", "document_ids")
        .expression_attribute_values(":document_ids_value", to_attribute_value(documents)?);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn add_document_to_collection(
    client: &Client,
    id: &str,
    document_id: &str,
) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression(
            "SET #attr_document_ids = list_append(#attr_document_ids, :document_id_value)",
        )
        .expression_attribute_names("#attr_document_ids", "document_ids")
        .expression_attribute_values(":document_id_value", to_attribute_value(vec![document_id])?);

    info!("Send update item request to dyanmodb {}", table);

    match request.send().await {
        Ok(_) => {
            info!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn remove_document_to_collection(
    client: &Client,
    id: &str,
    document_id: &str,
) -> Result<()> {
    let table = get_table_name()?;
    let request = client
        .update_item()
        .table_name(&table)
        .key("id", to_attribute_value(id)?)
        .update_expression("SET #attr_document_ids = list_remove(:document_id_value)")
        .expression_attribute_names("#attr_document_ids", "document_ids")
        .expression_attribute_values(":document_id_value", to_attribute_value(document_id)?);

    match request.send().await {
        Ok(_) => {
            println!("Update item to table: {}", table);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

pub async fn check_collection_permission(
    client: &Client,
    collection: &Collection,
    user_id: &str,
) -> Result<()> {
    if collection.creator_id() == user_id {
        return Ok(());
    }

    let group = get_group(client, &collection.group_id()).await?;
    if group.owner_ids().contains(&user_id.to_string()) {
        return Ok(());
    }

    Err(anyhow::anyhow!("Access Denied"))
}
