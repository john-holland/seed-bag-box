use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    ContaminationEvent, EnvironmentalConditions, GrowthStage, GreenhouseZone, Plant,
    PlantHealthStatus, QuarantineStatus, RiskLevel, SpatialCoordinates, ZoneType,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateZoneRequest {
    greenhouse_id: Uuid,
    zone_type: ZoneType,
    x: u32,
    y: u32,
    level: u32,
    isolation_distance_meters: Option<f32>,
    phenotype_designation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlantSeedRequest {
    seed_id: Uuid,
    zone_id: Uuid,
    species: String,
    variety: Option<String>,
    phenotype_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuarantineRequest {
    zone_id: Uuid,
    reason: String,
    contamination_type: String,
    severity: RiskLevel,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ZoneResponse {
    zone_id: Uuid,
    zone_type: ZoneType,
    quarantine_status: QuarantineStatus,
    plant_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlantResponse {
    plant_id: Uuid,
    species: String,
    zone_id: Uuid,
    growth_stage: GrowthStage,
    health_status: PlantHealthStatus,
}

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing greenhouse request");
    
    let path = event.payload.path.as_ref().map(|s| s.as_str()).unwrap_or("");
    let method = event.payload.http_method;
    
    let response = match (method.as_str(), path) {
        ("POST", "/greenhouse/zones") => create_zone(event.payload).await?,
        ("POST", "/greenhouse/plants") => plant_seed(event.payload).await?,
        ("POST", "/greenhouse/quarantine") => initiate_quarantine(event.payload).await?,
        ("GET", "/greenhouse/zones") => list_zones().await?,
        ("GET", "/greenhouse/plants") => list_plants().await?,
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

async fn create_zone(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: CreateZoneRequest = serde_json::from_str(&body)?;
    
    let zone_id = Uuid::new_v4();
    
    // Calculate contamination risk based on zone type and spatial parameters
    let contamination_risk = match req.zone_type {
        ZoneType::Quarantine => RiskLevel::High,
        ZoneType::PhenotypeIsolation => RiskLevel::Medium,
        ZoneType::Standard => RiskLevel::Low,
        _ => RiskLevel::Low,
    };
    
    let zone = GreenhouseZone {
        id: zone_id,
        zone_number: 0, // Will be assigned based on existing zones
        zone_type: req.zone_type.clone(),
        spatial_coordinates: SpatialCoordinates {
            x: req.x,
            y: req.y,
            level: req.level,
            isolation_distance_meters: req.isolation_distance_meters,
        },
        current_plants: vec![],
        quarantine_status: QuarantineStatus::None,
        phenotype_designation: req.phenotype_designation,
        environmental_conditions: EnvironmentalConditions {
            temperature_celsius: None,
            humidity_percent: None,
            light_hours_per_day: None,
            co2_ppm: None,
            last_measured: chrono::Utc::now(),
        },
        contamination_risk_level: contamination_risk,
    };
    
    info!("Created zone {} of type {:?} at coordinates ({}, {}, {})", 
        zone_id, zone.zone_type, req.x, req.y, req.level);
    
    // TODO: Save to DynamoDB
    // TODO: Validate spatial isolation requirements
    
    let response = ZoneResponse {
        zone_id: zone.id,
        zone_type: zone.zone_type,
        quarantine_status: zone.quarantine_status,
        plant_count: zone.current_plants.len(),
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn plant_seed(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: PlantSeedRequest = serde_json::from_str(&body)?;
    
    let plant_id = Uuid::new_v4();
    
    let plant = Plant {
        id: plant_id,
        seed_id: Some(req.seed_id),
        species: req.species.clone(),
        variety: req.variety,
        current_zone_id: req.zone_id,
        planted_at: chrono::Utc::now(),
        growth_stage: GrowthStage::Germination,
        health_status: PlantHealthStatus::Healthy,
        expected_harvest_date: None,
        phenotype_notes: req.phenotype_notes,
        contamination_history: vec![],
    };
    
    info!("Planted seed {} as plant {} in zone {}", req.seed_id, plant_id, req.zone_id);
    
    // TODO: Save to DynamoDB
    // TODO: Update zone's plant list
    // TODO: Check if zone requires preventive quarantine
    
    let response = PlantResponse {
        plant_id: plant.id,
        species: plant.species,
        zone_id: plant.current_zone_id,
        growth_stage: plant.growth_stage,
        health_status: plant.health_status,
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn initiate_quarantine(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: QuarantineRequest = serde_json::from_str(&body)?;
    
    info!("Initiating quarantine for zone {} due to {}", req.zone_id, req.reason);
    
    // TODO: Update zone quarantine status
    // TODO: Log contamination event for all plants in zone
    // TODO: Notify administrators
    // TODO: Calculate spatial impact on adjacent zones
    
    let event = ContaminationEvent {
        detected_at: chrono::Utc::now(),
        contamination_type: req.contamination_type,
        severity: req.severity,
        action_taken: format!("Zone {} quarantined: {}", req.zone_id, req.reason),
        resolved_at: None,
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&event)?)),
        is_base64_encoded: false,
    })
}

async fn list_zones() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing all greenhouse zones");
    
    // TODO: Query DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"zones": []}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn list_plants() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing all plants");
    
    // TODO: Query DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"plants": []}"#.to_string())),
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

