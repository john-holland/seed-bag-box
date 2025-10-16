use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    BagPackagingInstructions, LegStatus, ShipmentCycle, ShipmentLeg, ShipmentLegType,
    ShipmentStatus, ShipStationAddress, ShipStationCreateLabelRequest, Weight,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateShipmentRequest {
    customer_id: Uuid,
    subscription_id: Uuid,
    customer_address: models::Address,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShipmentResponse {
    shipment_id: Uuid,
    customer_to_facility_tracking: Option<String>,
    facility_to_customer_tracking: Option<String>,
    status: ShipmentStatus,
    packaging_instructions: BagPackagingInstructions,
}

/// Main Lambda handler for shipping and logistics
async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing shipping request");
    
    let path = event.payload.path.as_ref().map(|s| s.as_str()).unwrap_or("");
    let method = event.payload.http_method;
    
    let response = match (method.as_str(), path) {
        ("POST", "/shipments") => create_multi_point_shipment(event.payload).await?,
        ("GET", "/shipments/packaging-instructions") => get_packaging_instructions().await?,
        ("POST", path) if path.starts_with("/shipments/") && path.ends_with("/webhook") => {
            handle_shipstation_webhook(event.payload).await?
        }
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

async fn create_multi_point_shipment(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: CreateShipmentRequest = serde_json::from_str(&body)?;
    
    info!("Creating multi-point shipment for customer {}", req.customer_id);
    
    let shipment_id = Uuid::new_v4();
    let facility_address = models::Address {
        street1: "123 Seed Processing Center".to_string(),
        street2: None,
        city: "Portland".to_string(),
        state: "OR".to_string(),
        zip: "97201".to_string(),
        country: "US".to_string(),
    };
    
    // Leg 1: Customer to Facility (initial shipment of bags)
    let leg1 = ShipmentLeg {
        leg_number: 1,
        leg_type: ShipmentLegType::CustomerToFacility,
        from_address: req.customer_address.clone(),
        to_address: facility_address.clone(),
        shipstation_label_id: None,
        tracking_number: None,
        status: LegStatus::Pending,
        shipped_at: None,
        delivered_at: None,
    };
    
    // Leg 2: Facility to Customer (return of cleaned bags - FINAL LEG with return label logic)
    let leg2 = ShipmentLeg {
        leg_number: 2,
        leg_type: ShipmentLegType::FacilityToCustomer,
        from_address: facility_address.clone(),
        to_address: req.customer_address.clone(),
        shipstation_label_id: None,
        tracking_number: None,
        status: LegStatus::Pending,
        shipped_at: None,
        delivered_at: None,
    };
    
    let shipment = ShipmentCycle {
        id: shipment_id,
        customer_id: req.customer_id,
        subscription_id: req.subscription_id,
        legs: vec![leg1, leg2],
        status: ShipmentStatus::Pending,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Generate ShipStation labels for both legs
    // The return label (leg 2) is generated upfront but used as the final leg
    let customer_to_facility_label = create_shipstation_label(
        &shipment.id,
        &req.customer_address,
        &facility_address,
        false,
    ).await;
    
    let facility_to_customer_label = create_shipstation_label(
        &shipment.id,
        &facility_address,
        &req.customer_address,
        true, // This is the return label
    ).await;
    
    info!("Created multi-point shipment with return label as final leg");
    
    let response = ShipmentResponse {
        shipment_id: shipment.id,
        customer_to_facility_tracking: Some("MOCK_TRACKING_001".to_string()),
        facility_to_customer_tracking: Some("MOCK_TRACKING_002".to_string()),
        status: shipment.status,
        packaging_instructions: BagPackagingInstructions::trapezoid_butterfly_method(),
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn create_shipstation_label(
    shipment_id: &Uuid,
    from: &models::Address,
    to: &models::Address,
    is_return: bool,
) -> Result<(), Error> {
    info!("Creating ShipStation label for shipment {} (return: {})", shipment_id, is_return);
    
    let label_request = ShipStationCreateLabelRequest {
        order_id: shipment_id.to_string(),
        carrier_code: "usps".to_string(),
        service_code: "usps_priority_mail".to_string(),
        confirmation: "delivery".to_string(),
        ship_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        weight: Weight {
            value: 8.0,
            units: "ounces".to_string(),
        },
        dimensions: None,
        ship_from: ShipStationAddress {
            name: "Seed Box Bag Box".to_string(),
            street1: from.street1.clone(),
            street2: from.street2.clone(),
            city: from.city.clone(),
            state: from.state.clone(),
            postal_code: from.zip.clone(),
            country: from.country.clone(),
        },
        ship_to: ShipStationAddress {
            name: "Customer".to_string(),
            street1: to.street1.clone(),
            street2: to.street2.clone(),
            city: to.city.clone(),
            state: to.state.clone(),
            postal_code: to.zip.clone(),
            country: to.country.clone(),
        },
        is_return_label: is_return,
    };
    
    // TODO: Make actual API call to ShipStation
    // let client = reqwest::Client::new();
    // let response = client.post("https://ssapi.shipstation.com/shipments/createlabel")
    //     .basic_auth(api_key, Some(api_secret))
    //     .json(&label_request)
    //     .send()
    //     .await?;
    
    Ok(())
}

async fn get_packaging_instructions() -> Result<ApiGatewayProxyResponse, Error> {
    let instructions = BagPackagingInstructions::trapezoid_butterfly_method();
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&instructions)?)),
        is_base64_encoded: false,
    })
}

async fn handle_shipstation_webhook(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Received ShipStation webhook");
    
    // TODO: Parse webhook payload
    // TODO: Update shipment status in DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"status": "received"}"#.to_string())),
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

