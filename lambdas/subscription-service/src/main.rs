use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{Customer, Subscription, SubscriptionStatus, SubscriptionTier};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSubscriptionRequest {
    customer_email: String,
    customer_name: String,
    tier: SubscriptionTier,
    shipping_address: models::Address,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubscriptionResponse {
    subscription_id: Uuid,
    customer_id: Uuid,
    tier: SubscriptionTier,
    status: SubscriptionStatus,
    monthly_price_cents: u32,
    bags_required: bool,
    next_billing_date: String,
}

/// Main Lambda handler for subscription management
async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing subscription request");
    
    let path = event.payload.path.as_ref().map(|s| s.as_str()).unwrap_or("");
    let method = event.payload.http_method;
    
    let response = match (method.as_str(), path) {
        ("POST", "/subscriptions") => create_subscription(event.payload).await?,
        ("GET", path) if path.starts_with("/subscriptions/") => get_subscription(path).await?,
        ("PUT", path) if path.starts_with("/subscriptions/") => update_subscription(event.payload, path).await?,
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

async fn create_subscription(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: CreateSubscriptionRequest = serde_json::from_str(&body)?;
    
    // Create customer
    let customer_id = Uuid::new_v4();
    let customer = Customer {
        id: customer_id,
        email: req.customer_email,
        name: req.customer_name,
        shipping_address: req.shipping_address,
        created_at: chrono::Utc::now(),
        cratejoy_customer_id: None,
    };
    
    // Create subscription
    let subscription_id = Uuid::new_v4();
    let bags_required = req.tier.requires_bags();
    let initial_status = if bags_required {
        SubscriptionStatus::PendingBags
    } else {
        SubscriptionStatus::Active
    };
    
    let subscription = Subscription {
        id: subscription_id,
        customer_id,
        tier: req.tier.clone(),
        status: initial_status.clone(),
        cratejoy_subscription_id: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        next_billing_date: chrono::Utc::now() + chrono::Duration::days(30),
        bags_required,
    };
    
    // TODO: Save to DynamoDB
    // TODO: Create CrateJoy subscription
    // TODO: If bags required, trigger return label generation
    
    info!("Created subscription {} for customer {}", subscription_id, customer_id);
    
    let response = SubscriptionResponse {
        subscription_id: subscription.id,
        customer_id: subscription.customer_id,
        tier: subscription.tier.clone(),
        status: subscription.status,
        monthly_price_cents: subscription.tier.price_cents(),
        bags_required: subscription.bags_required,
        next_billing_date: subscription.next_billing_date.to_rfc3339(),
    };
    
    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn get_subscription(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    // Extract subscription ID from path
    let id = path.trim_start_matches("/subscriptions/");
    
    info!("Fetching subscription {}", id);
    
    // TODO: Fetch from DynamoDB
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(format!(r#"{{"subscription_id": "{}"}}"#, id))),
        is_base64_encoded: false,
    })
}

async fn update_subscription(request: ApiGatewayProxyRequest, path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let id = path.trim_start_matches("/subscriptions/");
    
    info!("Updating subscription {}", id);
    
    // TODO: Update in DynamoDB
    // TODO: Sync with CrateJoy
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"status": "updated"}"#.to_string())),
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

