use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Greenhouse management system with spatial quarantine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Greenhouse {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub total_zones: u32,
    pub zones: Vec<GreenhouseZone>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenhouseZone {
    pub id: Uuid,
    pub zone_number: u32,
    pub zone_type: ZoneType,
    pub spatial_coordinates: SpatialCoordinates,
    pub current_plants: Vec<Uuid>, // Plant IDs
    pub quarantine_status: QuarantineStatus,
    pub phenotype_designation: Option<String>, // For cannabis cultivation
    pub environmental_conditions: EnvironmentalConditions,
    pub contamination_risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneType {
    /// Standard growing zone
    Standard,
    /// Quarantine zone for potentially contaminated plants
    Quarantine,
    /// Isolation zone for specific phenotypes
    PhenotypeIsolation,
    /// Seed germination zone
    Germination,
    /// Harvest staging area
    HarvestStaging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialCoordinates {
    /// Grid position X
    pub x: u32,
    /// Grid position Y
    pub y: u32,
    /// Zone level (for multi-tier systems)
    pub level: u32,
    /// Minimum distance from other zones in meters
    pub isolation_distance_meters: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuarantineStatus {
    /// No quarantine, zone is operational
    None,
    /// Preventive quarantine for new plants
    Preventive,
    /// Active quarantine due to detected issues
    Active,
    /// Post-treatment monitoring
    Monitoring,
    /// Cleared and decontaminated
    Cleared,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConditions {
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    pub light_hours_per_day: Option<f32>,
    pub co2_ppm: Option<u32>,
    pub last_measured: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Plant tracking in greenhouse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plant {
    pub id: Uuid,
    pub seed_id: Option<Uuid>,
    pub species: String,
    pub variety: Option<String>,
    pub current_zone_id: Uuid,
    pub planted_at: DateTime<Utc>,
    pub growth_stage: GrowthStage,
    pub health_status: PlantHealthStatus,
    pub expected_harvest_date: Option<DateTime<Utc>>,
    pub phenotype_notes: Option<String>,
    pub contamination_history: Vec<ContaminationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrowthStage {
    Germination,
    Seedling,
    Vegetative,
    Flowering,
    Fruiting,
    Harvest,
    Curing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlantHealthStatus {
    Healthy,
    Monitoring,
    Diseased,
    Pest,
    Nutrient,
    Quarantine,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContaminationEvent {
    pub detected_at: DateTime<Utc>,
    pub contamination_type: String,
    pub severity: RiskLevel,
    pub action_taken: String,
    pub resolved_at: Option<DateTime<Utc>>,
}

