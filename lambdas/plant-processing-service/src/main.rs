use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use tracing::info;

use models::{CuringProtocol, PlantProcessingGuide, ProcessingType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProcessingGuideQuery {
    species: String,
    variety: Option<String>,
}

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing plant processing request");
    
    let path = event.payload.path.as_ref().map(|s| s.as_str()).unwrap_or("");
    let method = event.payload.http_method;
    
    let response = match (method.as_str(), path) {
        ("GET", "/processing/guides") => get_processing_guide(event.payload).await?,
        ("GET", "/processing/curing-protocols") => get_curing_protocol(event.payload).await?,
        ("GET", "/processing/recipes") => get_recipes(event.payload).await?,
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

async fn get_processing_guide(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let species = request
        .query_string_parameters
        .get("species")
        .ok_or("Missing species parameter")?;
    
    info!("Fetching processing guide for species: {}", species);
    
    // TODO: Query DynamoDB for species-specific guide
    // For now, return a sample guide
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(
            r#"{{"species": "{}", "is_edible": true, "processing_methods": []}}"#,
            species
        ))),
        is_base64_encoded: false,
    })
}

async fn get_curing_protocol(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let plant_type = request
        .query_string_parameters
        .get("plant_type")
        .ok_or("Missing plant_type parameter")?;
    
    info!("Fetching curing protocol for: {}", plant_type);
    
    // TODO: Query DynamoDB for curing protocols
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(
            r#"{{"plant_type": "{}", "phases": []}}"#,
            plant_type
        ))),
        is_base64_encoded: false,
    })
}

async fn get_recipes(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let species = request
        .query_string_parameters
        .get("species")
        .ok_or("Missing species parameter")?;
    
    info!("Fetching recipes for: {}", species);
    
    // TODO: Query DynamoDB for recipes
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(r#"{{"recipes": []}}"#))),
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

