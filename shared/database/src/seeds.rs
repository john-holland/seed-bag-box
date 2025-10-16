use aws_sdk_dynamodb::Client;
use uuid::Uuid;

use models::Seed;
use crate::{Result, DatabaseError, get_table_name};

pub struct SeedsRepository {
    client: Client,
    table_name: String,
}

impl SeedsRepository {
    pub fn new(client: Client) -> Self {
        let table_name = get_table_name("SEEDS_TABLE", "seed-box-seeds");
        Self { client, table_name }
    }

    pub async fn create(&self, seed: &Seed) -> Result<()> {
        // TODO: Implement DynamoDB put_item
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> Result<Seed> {
        // TODO: Implement DynamoDB get_item
        Err(DatabaseError::NotFound(id.to_string()))
    }

    pub async fn update(&self, seed: &Seed) -> Result<()> {
        // TODO: Implement DynamoDB update_item
        Ok(())
    }

    pub async fn list_by_status(&self, status: &str) -> Result<Vec<Seed>> {
        // TODO: Implement DynamoDB query using status-index GSI
        Ok(vec![])
    }

    pub async fn list_by_species(&self, species: &str) -> Result<Vec<Seed>> {
        // TODO: Implement DynamoDB scan with filter
        Ok(vec![])
    }
}

