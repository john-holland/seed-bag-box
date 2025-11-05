use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;

// Shared state for mock data
#[derive(Clone)]
struct AppState {
    scans: Arc<Mutex<Vec<Scan>>>,
    queue: Arc<Mutex<Vec<QueueItem>>>,
    seeds: Arc<Mutex<Vec<SeedStorage>>>,
    subscriptions: Arc<Mutex<Vec<Subscription>>>,
    images: Arc<Mutex<Vec<PlantImage>>>,
    audit_logs: Arc<Mutex<Vec<AuditLog>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scan {
    id: Uuid,
    code: String,
    scan_type: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueueItem {
    id: Uuid,
    queue_type: String,
    priority: String,
    status: String,
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SeedStorage {
    id: Uuid,
    seed_id: Uuid,
    species: String,
    location: String,
    temperature: f32,
    stored_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Subscription {
    id: Uuid,
    customer_id: Uuid,
    tier: String,
    status: String,
    price_cents: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlantImage {
    id: Uuid,
    uploaded_by: Uuid,
    item_id: Uuid,
    item_type: String,
    filename: String,
    s3_url: String,
    caption: Option<String>,
    moderation_status: String,
    uploaded_at: String,
    deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuditLog {
    id: Uuid,
    image_id: Uuid,
    action: String,
    performed_by: Uuid,
    performed_at: String,
    details: Option<String>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
struct ScanRequest {
    code: String,
    #[serde(rename = "type")]
    scan_type: String,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
struct CreateQueueRequest {
    queue_type: String,
    priority: String,
}

#[derive(Debug, Deserialize)]
struct StoreSeedRequest {
    seed_id: Uuid,
    species: String,
    facility: String,
    room: String,
}

#[derive(Debug, Deserialize)]
struct CreateSubscriptionRequest {
    customer_email: String,
    customer_name: String,
    tier: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize state
    let state = AppState {
        scans: Arc::new(Mutex::new(Vec::new())),
        queue: Arc::new(Mutex::new(Vec::new())),
        seeds: Arc::new(Mutex::new(Vec::new())),
        subscriptions: Arc::new(Mutex::new(Vec::new())),
        images: Arc::new(Mutex::new(Vec::new())),
        audit_logs: Arc::new(Mutex::new(Vec::new())),
    };

    // Build router
    let app = Router::new()
        // Scanner endpoints
        .route("/api/scan", post(handle_scan))
        .route("/api/scans", get(list_scans))
        
        // Queue endpoints
        .route("/api/queue", post(create_queue))
        .route("/api/queue", get(list_queue))
        .route("/api/queue/:id/start", put(start_queue))
        .route("/api/queue/:id/complete", put(complete_queue))
        
        // Storage endpoints
        .route("/api/storage/seeds", post(store_seed))
        .route("/api/storage/seeds", get(list_storage))
        .route("/api/storage/guide/:species", get(get_storage_guide))
        
        // Subscription endpoints
        .route("/api/subscriptions", post(create_subscription))
        .route("/api/subscriptions/:id", get(get_subscription))
        
        // Germination endpoints
        .route("/api/germination/start", post(start_germination))
        .route("/api/germination/ready", get(list_ready_germination))
        
        // Greenhouse endpoints
        .route("/api/greenhouse/zones", get(list_zones))
        .route("/api/greenhouse/plants", get(list_plants))
        
        // Image endpoints
        .route("/images/request-upload", post(request_presigned_url))
        .route("/images/confirm-upload", post(confirm_upload))
        .route("/images/my-images", get(list_my_images))
        .route("/images/:id", axum::routing::delete(delete_image))
        .route("/images/pending-moderation", get(list_pending_moderation))
        .route("/images/moderate", post(moderate_image))
        .route("/images/:id/audit", get(get_image_audit))
        
        // Health check
        .route("/health", get(health_check))
        
        // Root/index page
        .route("/", get(index_page))
        
        // PACT verification endpoint
        .route("/api/pact", get(pact_contracts))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "127.0.0.1:3000";
    info!("🔫 Mock API Server starting on http://{}", addr);
    info!("📋 PACT contracts available at http://{}/api/pact", addr);
    info!("🏥 Health check at http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// HANDLERS

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "seed-box-bag-box-mock-api",
        "version": "0.1.0"
    }))
}

async fn index_page() -> axum::response::Html<String> {
    axum::response::Html(format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>🌱 Seed Box Bag Box - Mock API</title>
    <style>
        body {{ font-family: monospace; padding: 40px; background: #f5f5f5; }}
        h1 {{ color: #009688; }}
        .endpoint {{ background: #fff; padding: 15px; margin: 10px 0; border-left: 4px solid #009688; }}
        .method {{ color: #00BCD4; font-weight: bold; }}
        a {{ color: #009688; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>🔫 Seed Box Bag Box - Mock API Server</h1>
    <p>Server is running! Here are the available endpoints:</p>
    
    <h2>📋 Meta</h2>
    <div class="endpoint"><span class="method">GET</span> <a href="/health">/health</a> - Health check</div>
    <div class="endpoint"><span class="method">GET</span> <a href="/api/pact">/api/pact</a> - PACT contracts</div>
    
    <h2>🔫 Scanner</h2>
    <div class="endpoint"><span class="method">POST</span> /api/scan - Record barcode scan</div>
    <div class="endpoint"><span class="method">GET</span> /api/scans - List scans</div>
    
    <h2>📦 Queue</h2>
    <div class="endpoint"><span class="method">POST</span> /api/queue - Create queue item</div>
    <div class="endpoint"><span class="method">GET</span> /api/queue - List queue</div>
    
    <h2>🧊 Storage</h2>
    <div class="endpoint"><span class="method">POST</span> /api/storage/seeds - Store seed</div>
    <div class="endpoint"><span class="method">GET</span> /api/storage/seeds - List storage</div>
    
    <h2>📸 Images</h2>
    <div class="endpoint"><span class="method">POST</span> /images/request-upload - Request presigned URL</div>
    <div class="endpoint"><span class="method">POST</span> /images/confirm-upload - Confirm upload</div>
    <div class="endpoint"><span class="method">GET</span> /images/my-images - List my images</div>
    <div class="endpoint"><span class="method">GET</span> /images/pending-moderation - Pending images</div>
    
    <h2>🌐 Web Interfaces</h2>
    <p>Open these files in your browser (in the <code>web/</code> directory):</p>
    <ul>
        <li>🥑 <strong><a href="http://localhost:8080/onboarding.html">onboarding.html</a></strong> - Avocado Proletariat story</li>
        <li>🌱 <strong><a href="http://localhost:8080/plant-lookup.html">plant-lookup.html</a></strong> - Search plants</li>
        <li>⚙️ <strong><a href="http://localhost:8080/user-settings.html">user-settings.html</a></strong> - User preferences & cannabis opt-in</li>
        <li>🔫 <strong><a href="http://localhost:8080/manufacturing-queue.html">manufacturing-queue.html</a></strong> - Scanner interface</li>
        <li>📸 <strong><a href="http://localhost:8080/my-images.html">my-images.html</a></strong> - Upload photos</li>
        <li>✅ <strong><a href="http://localhost:8080/image-moderation.html">image-moderation.html</a></strong> - Moderate photos</li>
        <li>⚠️ <strong><a href="http://localhost:8080/recall-moderation.html">recall-moderation.html</a></strong> - Food safety recalls</li>
    </ul>
    
    <p style="margin-top: 40px; color: #009688;">
        <strong>Keep Portland Weird</strong> 🕷️🦇🐸💞
    </p>
</body>
</html>
    "#))
}

async fn handle_scan(
    State(state): State<AppState>,
    Json(payload): Json<ScanRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    info!("📦 Scan received: {} ({})", payload.code, payload.scan_type);
    
    let scan = Scan {
        id: Uuid::new_v4(),
        code: payload.code,
        scan_type: payload.scan_type,
        timestamp: payload.timestamp,
    };
    
    state.scans.lock().await.push(scan.clone());
    
    (StatusCode::CREATED, Json(serde_json::json!({
        "scan_id": scan.id,
        "code": scan.code,
        "type": scan.scan_type,
        "status": "processed",
        "timestamp": scan.timestamp
    })))
}

async fn list_scans(State(state): State<AppState>) -> Json<serde_json::Value> {
    let scans = state.scans.lock().await;
    Json(serde_json::json!({
        "scans": scans.clone(),
        "count": scans.len()
    }))
}

async fn create_queue(
    State(state): State<AppState>,
    Json(payload): Json<CreateQueueRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let queue_item = QueueItem {
        id: Uuid::new_v4(),
        queue_type: payload.queue_type,
        priority: payload.priority,
        status: "PENDING".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    state.queue.lock().await.push(queue_item.clone());
    
    (StatusCode::CREATED, Json(serde_json::json!({
        "queue_id": queue_item.id,
        "status": queue_item.status,
        "priority": queue_item.priority,
        "created_at": queue_item.created_at
    })))
}

async fn list_queue(State(state): State<AppState>) -> Json<serde_json::Value> {
    let queue = state.queue.lock().await;
    let pending = queue.iter().filter(|q| q.status == "PENDING").count();
    let in_progress = queue.iter().filter(|q| q.status == "IN_PROGRESS").count();
    
    Json(serde_json::json!({
        "queue_items": queue.clone(),
        "pending_count": pending,
        "in_progress_count": in_progress
    }))
}

async fn start_queue(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let mut queue = state.queue.lock().await;
    if let Some(item) = queue.iter_mut().find(|q| q.id == id) {
        item.status = "IN_PROGRESS".to_string();
    }
    
    Json(serde_json::json!({
        "queue_id": id,
        "status": "IN_PROGRESS"
    }))
}

async fn complete_queue(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let mut queue = state.queue.lock().await;
    if let Some(item) = queue.iter_mut().find(|q| q.id == id) {
        item.status = "COMPLETED".to_string();
    }
    
    Json(serde_json::json!({
        "queue_id": id,
        "status": "COMPLETED"
    }))
}

async fn store_seed(
    State(state): State<AppState>,
    Json(payload): Json<StoreSeedRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let storage = SeedStorage {
        id: Uuid::new_v4(),
        seed_id: payload.seed_id,
        species: payload.species.clone(),
        location: format!("{}/{}", payload.facility, payload.room),
        temperature: 5.0,
        stored_at: chrono::Utc::now().to_rfc3339(),
    };
    
    state.seeds.lock().await.push(storage.clone());
    
    (StatusCode::CREATED, Json(serde_json::json!({
        "storage_id": storage.id,
        "seed_id": storage.seed_id,
        "location": storage.location,
        "refrigeration": true,
        "temperature_range": "2°C - 8°C (optimal: 5°C)",
        "max_storage_days": 1460
    })))
}

async fn list_storage(State(state): State<AppState>) -> Json<serde_json::Value> {
    let seeds = state.seeds.lock().await;
    Json(serde_json::json!({
        "storage_locations": seeds.clone(),
        "total_seeds": seeds.len(),
        "refrigerated_count": seeds.len()
    }))
}

async fn get_storage_guide(Path(species): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "species": species,
        "storage_requirements": {
            "temperature_celsius": { "min": 2.0, "max": 8.0, "optimal": 5.0 },
            "humidity_percent": { "min": 20.0, "max": 40.0, "optimal": 30.0 },
            "light": "DARK",
            "refrigeration_required": true,
            "max_storage_days": 1460
        },
        "special_instructions": ["Store in airtight container", "Label with variety and date"]
    }))
}

async fn create_subscription(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubscriptionRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let price = match payload.tier.as_str() {
        "BRING_YOUR_OWN_BAGS" => 800,
        "STANDARD" => 1500,
        "PREMIUM" => 1900,
        _ => 1500,
    };
    
    let sub = Subscription {
        id: Uuid::new_v4(),
        customer_id: Uuid::new_v4(),
        tier: payload.tier,
        status: "ACTIVE".to_string(),
        price_cents: price,
    };
    
    state.subscriptions.lock().await.push(sub.clone());
    
    (StatusCode::CREATED, Json(serde_json::json!({
        "subscription_id": sub.id,
        "customer_id": sub.customer_id,
        "tier": sub.tier,
        "status": sub.status,
        "monthly_price_cents": sub.price_cents,
        "bags_required": sub.tier == "BRING_YOUR_OWN_BAGS"
    })))
}

async fn get_subscription(Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "subscription_id": id,
        "tier": "STANDARD",
        "status": "ACTIVE",
        "monthly_price_cents": 1500
    }))
}

async fn start_germination(Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::CREATED, Json(serde_json::json!({
        "germination_record_id": Uuid::new_v4(),
        "seed_id": payload.get("seed_id"),
        "species": payload.get("species"),
        "phase": "IMBIBITION",
        "estimated_ship_date": chrono::Utc::now() + chrono::Duration::days(14)
    })))
}

async fn list_ready_germination() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ready_count": 0,
        "records": []
    }))
}

async fn list_zones() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "zones": []
    }))
}

async fn list_plants() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "plants": []
    }))
}

async fn request_presigned_url(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let upload_id = Uuid::new_v4();
    let s3_key = format!("images/{}", upload_id);
    
    (StatusCode::OK, Json(serde_json::json!({
        "upload_id": upload_id,
        "presigned_url": format!("https://mock-s3-url/{}", s3_key),
        "s3_key": s3_key,
        "expires_in_seconds": 3600
    })))
}

async fn confirm_upload(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let image_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    let image = PlantImage {
        id: image_id,
        uploaded_by: user_id,
        item_id: Uuid::new_v4(),
        item_type: "plant".to_string(),
        filename: payload.get("metadata").and_then(|m| m.get("filename")).and_then(|f| f.as_str()).unwrap_or("image.jpg").to_string(),
        s3_url: format!("https://via.placeholder.com/400x300/C8E6C9/009688?text=Plant+Photo"),
        caption: None,
        moderation_status: "PENDING".to_string(),
        uploaded_at: chrono::Utc::now().to_rfc3339(),
        deleted_at: None,
    };
    
    state.images.lock().await.push(image.clone());
    
    // Audit log
    let audit = AuditLog {
        id: Uuid::new_v4(),
        image_id,
        action: "UPLOADED".to_string(),
        performed_by: user_id,
        performed_at: chrono::Utc::now().to_rfc3339(),
        details: Some(format!("Uploaded {}", image.filename)),
    };
    state.audit_logs.lock().await.push(audit);
    
    (StatusCode::CREATED, Json(serde_json::to_value(&image).unwrap()))
}

async fn list_my_images(State(state): State<AppState>) -> Json<serde_json::Value> {
    let images = state.images.lock().await;
    Json(serde_json::json!({
        "images": images.clone(),
        "count": images.len()
    }))
}

async fn delete_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let user_id = Uuid::new_v4();
    
    // Soft delete
    let mut images = state.images.lock().await;
    if let Some(image) = images.iter_mut().find(|img| img.id == id) {
        image.deleted_at = Some(chrono::Utc::now().to_rfc3339());
        image.moderation_status = "DELETED".to_string();
    }
    
    // Audit log
    let audit = AuditLog {
        id: Uuid::new_v4(),
        image_id: id,
        action: "DELETED".to_string(),
        performed_by: user_id,
        performed_at: chrono::Utc::now().to_rfc3339(),
        details: Some("User deleted their own image".to_string()),
    };
    state.audit_logs.lock().await.push(audit);
    
    Json(serde_json::json!({
        "status": "deleted",
        "audit_logged": true
    }))
}

async fn list_pending_moderation(State(state): State<AppState>) -> Json<serde_json::Value> {
    let images = state.images.lock().await;
    let pending: Vec<_> = images.iter()
        .filter(|img| img.moderation_status == "PENDING")
        .cloned()
        .collect();
    
    Json(serde_json::json!({
        "pending_images": pending,
        "count": pending.len()
    }))
}

async fn moderate_image(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let image_id = payload.get("image_id").and_then(|v| v.as_str()).unwrap_or("");
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("APPROVED");
    let moderator_id = Uuid::new_v4();
    
    // Update image
    let mut images = state.images.lock().await;
    if let Ok(id) = Uuid::parse_str(image_id) {
        if let Some(image) = images.iter_mut().find(|img| img.id == id) {
            image.moderation_status = status.to_string();
        }
    }
    
    // Audit log
    let audit = AuditLog {
        id: Uuid::new_v4(),
        image_id: Uuid::parse_str(image_id).unwrap_or_default(),
        action: status.to_string(),
        performed_by: moderator_id,
        performed_at: chrono::Utc::now().to_rfc3339(),
        details: payload.get("notes").and_then(|v| v.as_str()).map(String::from),
    };
    state.audit_logs.lock().await.push(audit);
    
    Json(serde_json::json!({
        "image_id": image_id,
        "status": status,
        "moderated_by": moderator_id,
        "audit_logged": true
    }))
}

async fn get_image_audit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let logs = state.audit_logs.lock().await;
    let image_logs: Vec<_> = logs.iter()
        .filter(|log| log.image_id == id)
        .cloned()
        .collect();
    
    Json(serde_json::json!({
        "image_id": id,
        "audit_logs": image_logs,
        "count": image_logs.len()
    }))
}

async fn pact_contracts() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "seed-box-bag-box",
        "version": "0.1.0",
        "contracts": {
            "scanner": {
                "POST /api/scan": {
                    "request": {
                        "body": {
                            "code": "string",
                            "type": "string",
                            "timestamp": "ISO8601"
                        }
                    },
                    "response": {
                        "status": 201,
                        "body": {
                            "scan_id": "uuid",
                            "code": "string",
                            "type": "string",
                            "status": "processed"
                        }
                    }
                },
                "GET /api/scans": {
                    "response": {
                        "status": 200,
                        "body": {
                            "scans": "array",
                            "count": "number"
                        }
                    }
                }
            },
            "queue": {
                "POST /api/queue": {
                    "request": {
                        "body": {
                            "queue_type": "string",
                            "priority": "string"
                        }
                    },
                    "response": {
                        "status": 201,
                        "body": {
                            "queue_id": "uuid",
                            "status": "string",
                            "priority": "string"
                        }
                    }
                },
                "GET /api/queue": {
                    "response": {
                        "status": 200,
                        "body": {
                            "queue_items": "array",
                            "pending_count": "number",
                            "in_progress_count": "number"
                        }
                    }
                }
            },
            "storage": {
                "POST /api/storage/seeds": {
                    "request": {
                        "body": {
                            "seed_id": "uuid",
                            "species": "string",
                            "facility": "string",
                            "room": "string"
                        }
                    },
                    "response": {
                        "status": 201,
                        "body": {
                            "storage_id": "uuid",
                            "seed_id": "uuid",
                            "location": "string",
                            "refrigeration": "boolean",
                            "temperature_range": "string"
                        }
                    }
                }
            },
            "subscriptions": {
                "POST /api/subscriptions": {
                    "request": {
                        "body": {
                            "customer_email": "email",
                            "customer_name": "string",
                            "tier": "enum[BRING_YOUR_OWN_BAGS, STANDARD, PREMIUM]"
                        }
                    },
                    "response": {
                        "status": 201,
                        "body": {
                            "subscription_id": "uuid",
                            "customer_id": "uuid",
                            "tier": "string",
                            "monthly_price_cents": "number"
                        }
                    }
                }
            }
        }
    }))
}

