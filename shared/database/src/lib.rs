use aws_sdk_dynamodb::Client;
use thiserror::Error;

pub mod bags;
pub mod seeds;
pub mod subscriptions;
pub mod shipments;
pub mod greenhouse;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Item not found: {0}")]
    NotFound(String),
    
    #[error("DynamoDB error: {0}")]
    DynamoDb(#[from] aws_sdk_dynamodb::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Get DynamoDB client configured for the current AWS environment
pub async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

/// Helper to get table name from environment variable or use default
pub fn get_table_name(env_var: &str, default: &str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| default.to_string())
}

