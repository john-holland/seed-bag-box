use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Plant/Seed image with moderation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantImage {
    pub id: Uuid,
    pub uploaded_by: Uuid, // User ID
    pub item_id: Uuid,     // Seed or Plant ID
    pub item_type: ImageItemType,
    
    /// S3 details
    pub s3_bucket: String,
    pub s3_key: String,
    pub s3_url: String,
    
    /// Metadata
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    
    /// Description and tags
    pub caption: Option<String>,
    pub growth_stage: Option<String>,
    pub tags: Vec<String>,
    
    /// Moderation
    pub moderation_status: ModerationStatus,
    pub moderation_notes: Option<String>,
    pub moderated_by: Option<Uuid>,
    pub moderated_at: Option<DateTime<Utc>>,
    
    /// Timestamps
    pub uploaded_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageItemType {
    Seed,
    Plant,
    Greenhouse,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModerationStatus {
    /// Awaiting moderation
    Pending,
    /// Approved by moderator
    Approved,
    /// Rejected by moderator
    Rejected,
    /// Flagged for review
    Flagged,
    /// User deleted (soft delete)
    Deleted,
}

/// Audit log entry for image actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAuditLog {
    pub id: Uuid,
    pub image_id: Uuid,
    pub action: ImageAction,
    pub performed_by: Uuid,
    pub performed_at: DateTime<Utc>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImageAction {
    Uploaded,
    Approved,
    Rejected,
    Flagged,
    Deleted,
    Restored,
    ViewedByModerator,
}

/// S3 presigned URL for upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUpload {
    pub upload_id: Uuid,
    pub presigned_url: String,
    pub s3_key: String,
    pub expires_at: DateTime<Utc>,
    pub max_size_bytes: u64,
    pub allowed_types: Vec<String>,
}

/// Image upload metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUploadMetadata {
    pub item_id: Uuid,
    pub item_type: ImageItemType,
    pub filename: String,
    pub content_type: String,
    pub caption: Option<String>,
    pub growth_stage: Option<String>,
    pub tags: Vec<String>,
}

