use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    HazardClass, RecallData, RecallImpactAssessment, RecallReview, RecallSource, RecallStatus,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchRecallsRequest {
    days_back: Option<u32>,
    source: Option<RecallSource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewRecallRequest {
    recall_id: Uuid,
    is_relevant: bool,
    affected_species: Vec<String>,
    notes: String,
    manual_check_required: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssessImpactRequest {
    recall_id: Uuid,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing recall service request");

    let path = event
        .payload
        .path
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    let method = event.payload.http_method.clone();
    let payload = event.payload;

    let response = match (method.as_str(), path.as_str()) {
        ("POST", "/recalls/fetch-usda") => fetch_usda_recalls().await?,
        ("POST", "/recalls/fetch-fda") => fetch_fda_recalls().await?,
        ("GET", "/recalls/new") => list_new_recalls().await?,
        ("POST", "/recalls/review") => review_recall(payload).await?,
        ("POST", "/recalls/assess-impact") => assess_impact(payload).await?,
        ("GET", "/recalls/affected-customers") => list_affected_customers().await?,
        ("POST", p) if p.starts_with("/recalls/") && p.ends_with("/notify") => {
            notify_customers(&p).await?
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

async fn fetch_usda_recalls() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Fetching recalls from USDA API");

    // TODO: Call actual USDA FSIS API
    // URL: https://www.fsis.usda.gov/fsis-content/api/recalls
    // Documentation: https://www.fsis.usda.gov/science-data/data-sets-visualizations/recalls-api

    let mock_recalls = vec![
        serde_json::json!({
            "recall_number": "USDA-2024-001",
            "recall_date": "2025-10-10",
            "product_description": "Organic Spinach",
            "reason_for_recall": "Potential Salmonella contamination",
            "company_name": "Green Farms Inc.",
            "distribution": "CA, OR, WA",
            "classification": "Class I"
        }),
    ];

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "source": "USDA",
            "recalls_found": mock_recalls.len(),
            "recalls": mock_recalls,
            "fetched_at": chrono::Utc::now()
        }).to_string())),
        is_base64_encoded: false,
    })
}

async fn fetch_fda_recalls() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Fetching recalls from FDA API");

    // TODO: Call actual FDA Enforcement API
    // URL: https://api.fda.gov/food/enforcement.json
    // Documentation: https://open.fda.gov/apis/food/enforcement/

    let mock_recalls = vec![
        serde_json::json!({
            "recall_number": "FDA-2024-123",
            "recall_initiation_date": "2025-10-12",
            "product_description": "Fresh Tomatoes",
            "reason_for_recall": "Potential Listeria monocytogenes",
            "recalling_firm": "Fresh Produce Co.",
            "distribution_pattern": "Nationwide",
            "classification": "Class II"
        }),
    ];

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "source": "FDA",
            "recalls_found": mock_recalls.len(),
            "recalls": mock_recalls,
            "fetched_at": chrono::Utc::now()
        }).to_string())),
        is_base64_encoded: false,
    })
}

async fn list_new_recalls() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing new recalls for review");

    // TODO: Query DynamoDB for recalls with status = NEW

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "new_recalls": [],
            "count": 0,
            "reminder": "⚠️ Manual USDA/FDA website check recommended"
        }).to_string())),
        is_base64_encoded: false,
    })
}

async fn review_recall(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: ReviewRecallRequest = serde_json::from_str(&body)?;

    let review_id = Uuid::new_v4();
    let review = RecallReview {
        id: review_id,
        recall_id: req.recall_id,
        reviewed_by: "moderator".to_string(), // TODO: Get from auth
        reviewed_at: chrono::Utc::now(),
        is_relevant: req.is_relevant,
        affects_our_products: req.is_relevant,
        affected_species: req.affected_species,
        notes: req.notes,
        next_action: if req.is_relevant {
            "Assess customer impact and notify".to_string()
        } else {
            "No action needed".to_string()
        },
        manual_check_required: req.manual_check_required,
        manual_check_completed: false,
        manual_check_notes: None,
    };

    info!(
        "Recall {} reviewed - relevant: {}",
        req.recall_id, req.is_relevant
    );

    // TODO: Save review to DynamoDB
    // TODO: Update recall status
    // TODO: If relevant, trigger impact assessment

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&review)?)),
        is_base64_encoded: false,
    })
}

async fn assess_impact(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: AssessImpactRequest = serde_json::from_str(&body)?;

    info!("Assessing customer impact for recall {}", req.recall_id);

    // TODO: Fetch recall details
    // TODO: Query customer database for matching:
    //   - Shipping addresses in affected states
    //   - Products matching affected species
    //   - Shipments in affected date range

    let assessment = RecallImpactAssessment {
        id: Uuid::new_v4(),
        recall_id: req.recall_id,
        assessed_at: chrono::Utc::now(),
        assessed_by: "system".to_string(),
        potentially_affected_customers: vec![],
        affected_by_state: vec![],
        total_customers_affected: 0,
        affected_seed_ids: vec![],
        affected_plant_ids: vec![],
        affected_shipment_ids: vec![],
        risk_level: models::recalls::RiskLevel::Low,
        requires_customer_notification: false,
        requires_product_removal: false,
        actions_taken: vec![],
        customer_notification_sent_at: None,
        products_quarantined_at: None,
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&assessment)?)),
        is_base64_encoded: false,
    })
}

async fn list_affected_customers() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing affected customers across all recalls");

    // TODO: Query all relevant recalls and aggregate affected customers

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"affected_customers": [], "count": 0}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn notify_customers(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let recall_id = path
        .trim_start_matches("/recalls/")
        .trim_end_matches("/notify");

    info!("Notifying customers about recall {}", recall_id);

    // TODO: Fetch affected customers
    // TODO: Send email notifications
    // TODO: Create notification records

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "recall_id": recall_id,
            "customers_notified": 0,
            "notification_sent_at": chrono::Utc::now()
        }).to_string())),
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

