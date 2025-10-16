use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Manufacturing queue for coordinating seed → greenhouse → shipment workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturingQueue {
    pub id: Uuid,
    pub queue_type: QueueType,
    pub priority: Priority,
    pub status: QueueStatus,
    pub created_at: DateTime<Utc>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub assigned_to: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueueType {
    /// Seeds received, need to be processed and stored
    SeedIntake { seed_ids: Vec<Uuid> },
    /// Seeds ready to germinate
    GerminationScheduled { seed_ids: Vec<Uuid>, customer_id: Uuid },
    /// Plants ready to move to different greenhouse zone
    GreenhouseTransfer { plant_ids: Vec<Uuid>, from_zone_id: Uuid, to_zone_id: Uuid },
    /// Plants ready for shipment
    ShipmentPrep { germination_record_ids: Vec<Uuid>, customer_id: Uuid },
    /// Bags received, need cleaning
    BagCleaning { bag_ids: Vec<Uuid> },
    /// Bags cleaned, ready for return shipment
    BagShipmentPrep { bag_ids: Vec<Uuid>, customer_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueueStatus {
    Pending,
    Scheduled,
    InProgress,
    Paused,
    Completed,
    Cancelled,
    Failed,
}

/// Seed storage requirements and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedStorage {
    pub id: Uuid,
    pub seed_id: Uuid,
    pub storage_location: StorageLocation,
    pub storage_requirements: StorageRequirements,
    pub stored_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
    pub condition: StorageCondition,
    pub quantity_grams: Option<f32>,
    pub viability_tested: bool,
    pub estimated_viability_percent: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub facility: String,
    pub room: String,
    pub unit: StorageUnit,
    pub shelf: Option<String>,
    pub bin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageUnit {
    /// Cold storage refrigerator (2-8°C)
    ColdRefrigerator { unit_number: u32 },
    /// Freezer storage (-18°C or below)
    Freezer { unit_number: u32 },
    /// Room temperature storage (18-24°C)
    RoomTemp { cabinet_number: u32 },
    /// Climate controlled vault
    ControlledVault { vault_number: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequirements {
    /// Optimal storage temperature in Celsius
    pub temperature_celsius: TemperatureRange,
    /// Optimal humidity percentage
    pub humidity_percent: HumidityRange,
    /// Light exposure requirement
    pub light: LightExposure,
    /// Needs refrigeration
    pub refrigeration_required: bool,
    /// Needs freezing
    pub freezing_required: bool,
    /// Maximum storage duration in days
    pub max_storage_days: Option<u32>,
    /// Requires separation from other species
    pub separation_required: bool,
    /// Quarantine requirement
    pub quarantine_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureRange {
    pub min: f32,
    pub max: f32,
    pub optimal: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumidityRange {
    pub min: f32,
    pub max: f32,
    pub optimal: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LightExposure {
    Dark,
    Minimal,
    Indirect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Compromised,
}

/// Species-specific seed storage parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedStorageGuide {
    pub species: String,
    pub variety: Option<String>,
    pub storage_requirements: StorageRequirements,
    pub special_instructions: Vec<String>,
}

impl SeedStorageGuide {
    /// Get recommended storage requirements for common species
    pub fn for_species(species: &str) -> Self {
        match species.to_lowercase().as_str() {
            "tomato" | "tomatoes" => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 40.0,
                        optimal: 30.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(1460), // 4 years
                    separation_required: false,
                    quarantine_days: Some(14),
                },
                special_instructions: vec![
                    "Store in airtight container".to_string(),
                    "Label with variety and collection date".to_string(),
                ],
            },
            "pepper" | "peppers" => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 40.0,
                        optimal: 30.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(730), // 2 years
                    separation_required: true, // Different pepper varieties can cross
                    quarantine_days: Some(14),
                },
                special_instructions: vec![
                    "Store different varieties separately".to_string(),
                    "Dry completely before storage".to_string(),
                ],
            },
            "lettuce" | "salad" => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 40.0,
                        optimal: 30.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(1095), // 3 years
                    separation_required: false,
                    quarantine_days: Some(7),
                },
                special_instructions: vec![
                    "Quick germinator, use fresh seeds when possible".to_string(),
                ],
            },
            "basil" => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 40.0,
                        optimal: 30.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(1825), // 5 years
                    separation_required: false,
                    quarantine_days: Some(7),
                },
                special_instructions: vec![
                    "Very viable when stored properly".to_string(),
                ],
            },
            "cannabis" | "marijuana" => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 30.0,
                        optimal: 25.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(730), // 2 years optimal
                    separation_required: true, // MUST separate phenotypes
                    quarantine_days: Some(30), // Extended quarantine
                },
                special_instructions: vec![
                    "⚠️ REQUIRES SEPARATE LICENSED FACILITY".to_string(),
                    "Strict phenotype separation required".to_string(),
                    "Vacuum seal recommended".to_string(),
                    "Track with state compliance system".to_string(),
                ],
            },
            _ => Self {
                species: species.to_string(),
                variety: None,
                storage_requirements: StorageRequirements {
                    temperature_celsius: TemperatureRange {
                        min: 2.0,
                        max: 8.0,
                        optimal: 5.0,
                    },
                    humidity_percent: HumidityRange {
                        min: 20.0,
                        max: 40.0,
                        optimal: 30.0,
                    },
                    light: LightExposure::Dark,
                    refrigeration_required: true,
                    freezing_required: false,
                    max_storage_days: Some(1095), // 3 years default
                    separation_required: false,
                    quarantine_days: Some(14),
                },
                special_instructions: vec![
                    "Standard seed storage protocol".to_string(),
                ],
            },
        }
    }
}

/// Greenhouse workflow queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenhouseWorkflow {
    pub id: Uuid,
    pub workflow_type: GreenhouseWorkflowType,
    pub plant_ids: Vec<Uuid>,
    pub current_zone_id: Option<Uuid>,
    pub target_zone_id: Option<Uuid>,
    pub status: QueueStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GreenhouseWorkflowType {
    /// Move to germination zone
    MoveToGermination,
    /// Move from germination to growing zone
    TransferToGrowing,
    /// Move to quarantine zone
    MoveToQuarantine,
    /// Release from quarantine
    ReleaseFromQuarantine,
    /// Move to harvest staging
    MoveToHarvestStaging,
    /// Prepare for customer shipment
    PrepareForShipment,
}

/// Inventory audit and quality check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAudit {
    pub id: Uuid,
    pub audit_type: AuditType,
    pub performed_at: DateTime<Utc>,
    pub performed_by: String,
    pub items_checked: u32,
    pub issues_found: u32,
    pub findings: Vec<AuditFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditType {
    SeedStorageCheck,
    RefrigerationTemperature,
    GreenhouseHealth,
    QuarantineCompliance,
    ShipmentQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub item_id: Uuid,
    pub item_type: String,
    pub issue: String,
    pub severity: Severity,
    pub action_taken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

