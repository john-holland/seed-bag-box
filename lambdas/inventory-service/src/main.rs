use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{Bag, BagCondition, BagStatus, BagType, Seed, SeedStatus};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReceiveBagRequest {
    customer_id: Option<Uuid>,
    bag_type: BagType,
    condition: BagCondition,
    contains_seeds: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterSeedRequest {
    plant_species: String,
    variety: Option<String>,
    source_customer_id: Option<Uuid>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BagResponse {
    bag_id: Uuid,
    status: BagStatus,
    received_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SeedResponse {
    seed_id: Uuid,
    species: String,
    status: SeedStatus,
    is_edible_fruit_bearing: Option<bool>,
}

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing inventory request");
    
    let path = event.payload.path.as_ref().map(|s| s.as_str()).unwrap_or("");
    let method = event.payload.http_method;
    
    let response = match (method.as_str(), path) {
        ("POST", "/inventory/bags") => receive_bag(event.payload).await?,
        ("POST", "/inventory/seeds") => register_seed(event.payload).await?,
        ("GET", "/inventory/bags") => list_bags().await?,
        ("GET", "/inventory/seeds") => list_seeds().await?,
        ("GET", "/inventory/summary") => get_inventory_summary().await?,
        _ => ApiGatewayProxyResponse {
            status_code: 404,
            headers: Default::default(),
            multi_value_headers: Default::default(),
            body: Some(Body::Text(r#"{"error": "Not found"}"#.to_string())),
            is_base64_encoded: false,
        },
    };
    
    Ok(response)
}

async fn receive_bag(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: ReceiveBagRequest = serde_json::from_str(&body)?;
    
    let bag_id = Uuid::new_v4();
    let bag = Bag {
        id: bag_id,
        original_owner_id: req.customer_id,
        current_status: BagStatus::Received,
        bag_type: req.bag_type,
        condition: req.condition,
        received_at: chrono::Utc::now(),
        cleaned_at: None,
        last_updated: chrono::Utc::now(),
        contains_seeds: req.contains_seeds,
        seed_ids: vec![],
    };
    
    info!("Received bag {} from customer {:?}", bag_id, req.customer_id);
    
    // TODO: Save to DynamoDB
    
    let response = BagResponse {
        bag_id: bag.id,
        status: bag.current_status,
        received_at: bag.received_at.to_rfc3339(),
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn register_seed(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: RegisterSeedRequest = serde_json::from_str(&body)?;
    
    let seed_id = Uuid::new_v4();
    let seed = Seed {
        id: seed_id,
        plant_species: req.plant_species.clone(),
        variety: req.variety,
        source_customer_id: req.source_customer_id,
        collected_at: chrono::Utc::now(),
        status: SeedStatus::Collected,
        germination_tested: false,
        germination_rate: None,
        is_edible_fruit_bearing: None, // Will be determined during testing
        contamination_check: None,
        notes: req.notes,
    };
    
    info!("Registered seed {} for species {}", seed_id, seed.plant_species);
    
    // TODO: Save to DynamoDB
    // TODO: Trigger contamination check
    
    let response = SeedResponse {
        seed_id: seed.id,
        species: seed.plant_species,
        status: seed.status,
        is_edible_fruit_bearing: seed.is_edible_fruit_bearing,
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn list_bags() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing all bags");
    
    // TODO: Query DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"bags": []}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn list_seeds() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing all seeds");
    
    // TODO: Query DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"seeds": []}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn get_inventory_summary() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Generating inventory summary");
    
    // TODO: Aggregate from DynamoDB
    
    let summary = models::InventorySummary {
        total_bags: 0,
        bags_by_status: vec![],
        total_seeds: 0,
        seeds_by_status: vec![],
        bags_ready_for_shipment: 0,
        premium_customer_bags_held: 0,
        random_sampling_pool_size: 0,
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&summary)?)),
        is_base64_encoded: false,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

