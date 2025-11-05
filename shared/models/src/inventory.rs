use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Bag inventory tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bag {
    pub id: Uuid,
    pub original_owner_id: Option<Uuid>, // None if anonymous donation
    pub current_status: BagStatus,
    pub bag_type: BagType,
    pub condition: BagCondition,
    pub received_at: DateTime<Utc>,
    pub cleaned_at: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub contains_seeds: bool,
    pub seed_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BagStatus {
    Received,
    Cleaning,
    Cleaned,
    Quarantine,
    ReadyForShipment,
    Shipped,
    Recycled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BagType {
    Plastic,
    Paper,
    Reusable,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BagCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Recyclable,
}

/// Seed collection and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub plant_species: String,
    pub variety: Option<String>,
    pub source_customer_id: Option<Uuid>,
    pub collected_at: DateTime<Utc>,
    pub status: SeedStatus,
    pub germination_tested: bool,
    pub germination_rate: Option<f32>,
    
    /// Edible parts indicators
    pub is_edible_fruit_bearing: Option<bool>,
    pub has_edible_leaves: Option<bool>,
    pub has_edible_stalks: Option<bool>,
    
    pub contamination_check: Option<ContaminationCheck>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SeedStatus {
    Collected,
    Testing,
    Approved,
    Quarantine,
    Rejected,
    Planted,
    Distributed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContaminationCheck {
    pub checked_at: DateTime<Utc>,
    pub is_contaminated: bool,
    pub contamination_type: Option<String>,
    pub notes: Option<String>,
}

/// Inventory summary and reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySummary {
    pub total_bags: u32,
    pub bags_by_status: Vec<StatusCount>,
    pub total_seeds: u32,
    pub seeds_by_status: Vec<StatusCount>,
    pub bags_ready_for_shipment: u32,
    pub premium_customer_bags_held: u32,
    pub random_sampling_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCount {
    pub status: String,
    pub count: u32,
}

