use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use models::{
    ImageAction, ImageAuditLog, ImageItemType, ImageUploadMetadata, ModerationStatus,
    PlantImage, PresignedUpload,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestPresignedUrlRequest {
    item_id: Uuid,
    item_type: ImageItemType,
    filename: String,
    content_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfirmUploadRequest {
    upload_id: Uuid,
    metadata: ImageUploadMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModerateImageRequest {
    image_id: Uuid,
    status: ModerationStatus,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PresignedUrlResponse {
    upload_id: Uuid,
    presigned_url: String,
    s3_key: String,
    expires_in_seconds: u64,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Processing image service request");

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
        // Upload flow
        ("POST", "/images/request-upload") => request_presigned_url(payload).await?,
        ("POST", "/images/confirm-upload") => confirm_upload(payload).await?,
        
        // User image management
        ("GET", "/images/my-images") => list_my_images(payload).await?,
        ("DELETE", p) if p.starts_with("/images/") => delete_my_image(p, payload).await?,
        
        // Moderation
        ("GET", "/images/pending-moderation") => list_pending_moderation().await?,
        ("POST", "/images/moderate") => moderate_image(payload).await?,
        
        // Audit logs
        ("GET", "/images/audit-log") => get_audit_log(payload).await?,
        ("GET", p) if p.starts_with("/images/") && p.ends_with("/audit") => {
            get_image_audit_log(p).await?
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

async fn request_presigned_url(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: RequestPresignedUrlRequest = serde_json::from_str(&body)?;

    let upload_id = Uuid::new_v4();
    let s3_key = format!(
        "images/{}/{}/{}",
        item_type_to_string(&req.item_type),
        req.item_id,
        upload_id
    );

    info!(
        "Generating presigned URL for upload {} ({})",
        upload_id, req.filename
    );

    // TODO: Generate actual S3 presigned URL
    let presigned_url = format!("https://mock-s3-presigned-url/{}", s3_key);

    let response = PresignedUrlResponse {
        upload_id,
        presigned_url,
        s3_key,
        expires_in_seconds: 3600, // 1 hour
    };

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&response)?)),
        is_base64_encoded: false,
    })
}

async fn confirm_upload(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: ConfirmUploadRequest = serde_json::from_str(&body)?;

    let image_id = Uuid::new_v4();
    let user_id = Uuid::new_v4(); // TODO: Get from auth

    let image = PlantImage {
        id: image_id,
        uploaded_by: user_id,
        item_id: req.metadata.item_id,
        item_type: req.metadata.item_type,
        s3_bucket: "seed-box-images".to_string(),
        s3_key: format!("images/{}", req.upload_id),
        s3_url: format!("https://s3.amazonaws.com/seed-box-images/images/{}", req.upload_id),
        filename: req.metadata.filename,
        content_type: req.metadata.content_type,
        size_bytes: 0, // TODO: Get from S3
        width: None,
        height: None,
        caption: req.metadata.caption,
        growth_stage: req.metadata.growth_stage,
        tags: req.metadata.tags,
        moderation_status: ModerationStatus::Pending,
        moderation_notes: None,
        moderated_by: None,
        moderated_at: None,
        uploaded_at: chrono::Utc::now(),
        deleted_at: None,
        deleted_by: None,
    };

    // Create audit log entry
    let audit = ImageAuditLog {
        id: Uuid::new_v4(),
        image_id,
        action: ImageAction::Uploaded,
        performed_by: user_id,
        performed_at: chrono::Utc::now(),
        details: Some(format!("Uploaded {}", image.filename)),
        ip_address: None,
        user_agent: None,
    };

    info!("Image {} uploaded by user {}", image_id, user_id);

    // TODO: Save to DynamoDB
    // TODO: Save audit log

    Ok(ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string(&image)?)),
        is_base64_encoded: false,
    })
}

async fn list_my_images(
    _request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    // TODO: Get user ID from auth
    // TODO: Query DynamoDB for user's images

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"images": [], "count": 0}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn delete_my_image(
    path: &str,
    _request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let image_id = path.trim_start_matches("/images/");
    let user_id = Uuid::new_v4(); // TODO: Get from auth

    info!("User {} deleting image {}", user_id, image_id);

    // Soft delete - mark as deleted but keep in DB
    // Create audit log entry
    let audit = ImageAuditLog {
        id: Uuid::new_v4(),
        image_id: Uuid::parse_str(image_id).unwrap_or_default(),
        action: ImageAction::Deleted,
        performed_by: user_id,
        performed_at: chrono::Utc::now(),
        details: Some("User deleted their own image".to_string()),
        ip_address: None,
        user_agent: None,
    };

    // TODO: Update image in DynamoDB (soft delete)
    // TODO: Save audit log

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(
            r#"{"status": "deleted", "audit_logged": true}"#.to_string(),
        )),
        is_base64_encoded: false,
    })
}

async fn list_pending_moderation() -> Result<ApiGatewayProxyResponse, Error> {
    info!("Listing images pending moderation");

    // TODO: Query DynamoDB for pending images

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(
            r#"{"pending_images": [], "count": 0}"#.to_string(),
        )),
        is_base64_encoded: false,
    })
}

async fn moderate_image(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = request.body.ok_or("Missing body")?;
    let req: ModerateImageRequest = serde_json::from_str(&body)?;

    let moderator_id = Uuid::new_v4(); // TODO: Get from auth

    info!(
        "Moderator {} setting image {} to {:?}",
        moderator_id, req.image_id, req.status
    );

    // Create audit log entry
    let action = match req.status {
        ModerationStatus::Approved => ImageAction::Approved,
        ModerationStatus::Rejected => ImageAction::Rejected,
        ModerationStatus::Flagged => ImageAction::Flagged,
        _ => ImageAction::ViewedByModerator,
    };

    let audit = ImageAuditLog {
        id: Uuid::new_v4(),
        image_id: req.image_id,
        action,
        performed_by: moderator_id,
        performed_at: chrono::Utc::now(),
        details: req.notes.clone(),
        ip_address: None,
        user_agent: None,
    };

    // TODO: Update image in DynamoDB
    // TODO: Save audit log

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "image_id": req.image_id,
            "status": req.status,
            "moderated_by": moderator_id,
            "audit_logged": true
        }).to_string())),
        is_base64_encoded: false,
    })
}

async fn get_audit_log(
    _request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Fetching audit log");

    // TODO: Query DynamoDB for audit logs

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(r#"{"audit_logs": [], "count": 0}"#.to_string())),
        is_base64_encoded: false,
    })
}

async fn get_image_audit_log(path: &str) -> Result<ApiGatewayProxyResponse, Error> {
    let image_id = path
        .trim_start_matches("/images/")
        .trim_end_matches("/audit");

    info!("Fetching audit log for image {}", image_id);

    // TODO: Query DynamoDB for image-specific audit logs

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::json!({
            "image_id": image_id,
            "audit_logs": [],
            "count": 0
        }).to_string())),
        is_base64_encoded: false,
    })
}

fn item_type_to_string(item_type: &ImageItemType) -> String {
    match item_type {
        ImageItemType::Seed => "seed".to_string(),
        ImageItemType::Plant => "plant".to_string(),
        ImageItemType::Greenhouse => "greenhouse".to_string(),
        ImageItemType::General => "general".to_string(),
    }
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

