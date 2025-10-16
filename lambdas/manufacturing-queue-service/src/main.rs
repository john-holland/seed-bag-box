use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    GreenhouseWorkflow, GreenhouseWorkflowType, ManufacturingQueue, Priority, QueueStatus,
    QueueType, SeedStorage, SeedStorageGuide, StorageCondition, StorageLocation, StorageUnit,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateQueueRequest {
    queue_type: QueueType,
    priority: Priority,
    scheduled_start: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoreSeedRequest {
    seed_id: Uuid,
    species: String,
    facility: String,
    room: String,
    unit_number: u32,
    quantity_grams: Option<f32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QueueResponse {
    queue_id: Uuid,
    status: QueueStatus,
    priority: Priority,
    created_at: String,
    estimated_completion: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StorageResponse {
    storage_id: Uuid,
    seed_id: Uuid,
    location: String,
    refrigeration: bool,
    temperature_range: String,
    max_storage_days: Option<u32>,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing manufacturing queue request");

    let path = event
        .payload
        .path
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("");
    let method = event.payload.http_method;

    let response = match (method.as_str(), path) {
        ("POST", "/queue") => create_queue_item(event.payload).await?,
        ("GET", "/queue") => list_queue().await?,
        ("PUT", path) if path.starts_with("/queue/") && path.ends_with("/start") => {
            start_queue_item(path).await?
        }
        ("PUT", path) if path.starts_with("/queue/") && path.ends_with("/complete") => {
            complete_queue_item(path).await?
        }
        ("POST", "/storage/seeds") => store_seed(event.payload).await?,
        ("GET", "/storage/seeds") => list_seed_storage().await?,
        ("GET", path) if path.starts_with("/storage/guide/") => get_storage_guide(path).await?,
        ("GET", "/greenhouse/workflow") => list_greenhouse_workflows().await?,
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

async fn create_queue_item(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: CreateQueueRequest = serde_json::from_str(&body)?;

    let queue_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let scheduled = req
        .scheduled_start
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let queue = ManufacturingQueue {
        id: queue_id,
        queue_type: req.queue_type.clone(),
        priority: req.priority.clone(),
        status: QueueStatus::Pending,
        created_at: now,
        scheduled_start: scheduled,
        actual_start: None,
        completed_at: None,
        assigned_to: None,
        notes: None,
    };

    info!(
        "Created queue item {} with priority {:?}",
        queue_id, req.priority
    );

    // TODO: Save to DynamoDB

    let response = QueueResponse {
        queue_id: queue.id,
        status: queue.status,
        priority: queue.priority,
        created_at: queue.created_at.to_rfc3339(),
        estimated_completion: None, // TODO: Calculate based on queue
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn list_queue() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing manufacturing queue");

    // TODO: Query DynamoDB, sort by priority and scheduled time

    let response = serde_json::json!({
        "queue_items": [],
        "pending_count": 0,
        "in_progress_count": 0,
    });

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn start_queue_item(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let id = path
        .trim_start_matches("/queue/")
        .trim_end_matches("/start");

    info!("Starting queue item {}", id);

    // TODO: Update status to InProgress in DynamoDB

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(
            r#"{{"queue_id": "{}", "status": "IN_PROGRESS"}}"#,
            id
        ))),
        is_base64_encoded: false,
    })
}

async fn complete_queue_item(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let id = path
        .trim_start_matches("/queue/")
        .trim_end_matches("/complete");

    info!("Completing queue item {}", id);

    // TODO: Update status to Completed in DynamoDB
    // TODO: Trigger next workflow step if applicable

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(
            r#"{{"queue_id": "{}", "status": "COMPLETED"}}"#,
            id
        ))),
        is_base64_encoded: false,
    })
}

async fn store_seed(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: StoreSeedRequest = serde_json::from_str(&body)?;

    let storage_id = Uuid::new_v4();

    // Get storage requirements for species
    let guide = SeedStorageGuide::for_species(&req.species);

    let storage = SeedStorage {
        id: storage_id,
        seed_id: req.seed_id,
        storage_location: StorageLocation {
            facility: req.facility,
            room: req.room,
            unit: if guide.storage_requirements.refrigeration_required {
                StorageUnit::ColdRefrigerator {
                    unit_number: req.unit_number,
                }
            } else {
                StorageUnit::RoomTemp {
                    cabinet_number: req.unit_number,
                }
            },
            shelf: None,
            bin: None,
        },
        storage_requirements: guide.storage_requirements.clone(),
        stored_at: chrono::Utc::now(),
        last_checked: chrono::Utc::now(),
        condition: StorageCondition::Excellent,
        quantity_grams: req.quantity_grams,
        viability_tested: false,
        estimated_viability_percent: None,
    };

    info!(
        "Stored seed {} ({}) in {:?}",
        req.seed_id, req.species, storage.storage_location.unit
    );

    // TODO: Save to DynamoDB
    // TODO: Create queue item for quarantine period

    let temp_range = format!(
        "{}°C - {}°C (optimal: {}°C)",
        storage.storage_requirements.temperature_celsius.min,
        storage.storage_requirements.temperature_celsius.max,
        storage.storage_requirements.temperature_celsius.optimal
    );

    let response = StorageResponse {
        storage_id: storage.id,
        seed_id: storage.seed_id,
        location: format!(
            "{} / {} / {:?}",
            storage.storage_location.facility,
            storage.storage_location.room,
            storage.storage_location.unit
        ),
        refrigeration: storage.storage_requirements.refrigeration_required,
        temperature_range: temp_range,
        max_storage_days: storage.storage_requirements.max_storage_days,
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn list_seed_storage() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing seed storage");

    // TODO: Query DynamoDB

    let response = serde_json::json!({
        "storage_locations": [],
        "total_seeds": 0,
        "refrigerated_count": 0,
    });

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn get_storage_guide(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let species = path.trim_start_matches("/storage/guide/");

    info!("Fetching storage guide for {}", species);

    let guide = SeedStorageGuide::for_species(species);

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&guide)?)),
        is_base64_encoded: false,
    })
}

async fn list_greenhouse_workflows() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing greenhouse workflows");

    // TODO: Query DynamoDB for pending workflows

    let response = serde_json::json!({
        "workflows": [],
        "pending_transfers": 0,
        "quarantine_releases": 0,
    });

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
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

