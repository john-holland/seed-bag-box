use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// USDA/FDA Recall from external API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallData {
    pub id: Uuid,
    pub external_id: String, // USDA/FDA recall number
    pub source: RecallSource,
    
    /// Product information
    pub product_name: String,
    pub product_description: String,
    pub company_name: String,
    pub brand_names: Vec<String>,
    
    /// Contamination details
    pub reason_for_recall: String,
    pub contamination_type: Option<String>, // "Salmonella", "E. coli", etc.
    pub hazard_classification: HazardClass,
    
    /// Distribution
    pub distribution_pattern: String, // "Nationwide" or specific states
    pub affected_states: Vec<String>,
    pub distribution_start_date: Option<DateTime<Utc>>,
    pub distribution_end_date: Option<DateTime<Utc>>,
    
    /// Recall metadata
    pub recall_date: DateTime<Utc>,
    pub recall_initiation_date: Option<DateTime<Utc>>,
    pub recall_number: String,
    pub recall_classification: String,
    
    /// Product identification
    pub upc_codes: Vec<String>,
    pub lot_codes: Vec<String>,
    pub affected_species: Vec<String>, // Plant species affected
    
    /// URLs and documents
    pub url: Option<String>,
    pub press_release_url: Option<String>,
    pub fda_recall_url: Option<String>,
    
    /// Our tracking
    pub imported_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
    pub status: RecallStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecallSource {
    UsdaFsis,  // USDA Food Safety and Inspection Service
    Fda,       // FDA
    Cdcnors,   // CDC National Outbreak Reporting System
    Manual,    // Manually entered
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HazardClass {
    ClassI,   // Serious adverse health consequences or death
    ClassII,  // Temporary or medically reversible adverse health consequences
    ClassIII, // Not likely to cause adverse health consequences
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecallStatus {
    /// New recall, not yet reviewed
    New,
    /// Under review by moderator
    UnderReview,
    /// Relevant to our operations - customers may be affected
    Relevant,
    /// Not relevant to our products
    NotRelevant,
    /// Customers have been notified
    CustomersNotified,
    /// Recall resolved/completed
    Resolved,
}

/// Customer impact assessment for a recall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallImpactAssessment {
    pub id: Uuid,
    pub recall_id: Uuid,
    pub assessed_at: DateTime<Utc>,
    pub assessed_by: String,
    
    /// Affected customers
    pub potentially_affected_customers: Vec<Uuid>,
    pub affected_by_state: Vec<StateCount>,
    pub total_customers_affected: u32,
    
    /// Affected products in our system
    pub affected_seed_ids: Vec<Uuid>,
    pub affected_plant_ids: Vec<Uuid>,
    pub affected_shipment_ids: Vec<Uuid>,
    
    /// Risk assessment
    pub risk_level: RiskLevel,
    pub requires_customer_notification: bool,
    pub requires_product_removal: bool,
    
    /// Actions taken
    pub actions_taken: Vec<String>,
    pub customer_notification_sent_at: Option<DateTime<Utc>>,
    pub products_quarantined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateCount {
    pub state: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Moderation review for a recall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallReview {
    pub id: Uuid,
    pub recall_id: Uuid,
    pub reviewed_by: String,
    pub reviewed_at: DateTime<Utc>,
    
    pub is_relevant: bool,
    pub affects_our_products: bool,
    pub affected_species: Vec<String>,
    pub notes: String,
    pub next_action: String,
    
    /// Reminders
    pub manual_check_required: bool,
    pub manual_check_completed: bool,
    pub manual_check_notes: Option<String>,
}

/// Customer notification for recall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallNotification {
    pub id: Uuid,
    pub recall_id: Uuid,
    pub customer_id: Uuid,
    pub sent_at: DateTime<Utc>,
    pub notification_type: NotificationType,
    pub message: String,
    pub acknowledged: bool,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Email,
    Sms,
    InApp,
}

/// USDA API response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsdaApiResponse {
    pub results: Vec<UsdaRecall>,
    pub total: u32,
    pub page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsdaRecall {
    pub recall_number: String,
    pub recall_date: String,
    pub product_description: String,
    pub reason_for_recall: String,
    pub company_name: String,
    pub distribution: String,
    pub classification: String,
}

