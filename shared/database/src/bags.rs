use aws_sdk_dynamodb::Client;
use uuid::Uuid;

use models::Bag;
use crate::{Result, DatabaseError, get_table_name};

pub struct BagsRepository {
    client: Client,
    table_name: String,
}

impl BagsRepository {
    pub fn new(client: Client) -> Self {
        let table_name = get_table_name("BAGS_TABLE", "seed-box-bags");
        Self { client, table_name }
    }

    pub async fn create(&self, bag: &Bag) -> Result<()> {
        // TODO: Implement DynamoDB put_item
        // Convert Bag to DynamoDB AttributeValue map
        // Call client.put_item()
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> Result<Bag> {
        // TODO: Implement DynamoDB get_item
        // Call client.get_item()
        // Convert AttributeValue map to Bag
        Err(DatabaseError::NotFound(id.to_string()))
    }

    pub async fn update(&self, bag: &Bag) -> Result<()> {
        // TODO: Implement DynamoDB update_item
        Ok(())
    }

    pub async fn list_by_status(&self, status: &str) -> Result<Vec<Bag>> {
        // TODO: Implement DynamoDB query using status-index GSI
        Ok(vec![])
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        // TODO: Implement DynamoDB delete_item
        Ok(())
    }
}

