use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Contamination tracking and food safety management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContaminationReport {
    pub id: Uuid,
    pub report_type: ReportType,
    pub item_id: Uuid, // Can be seed_id, plant_id, or bag_id
    pub item_type: ItemType,
    pub contamination_type: ContaminationType,
    pub detected_at: DateTime<Utc>,
    pub severity: SeverityLevel,
    pub source: Option<String>,
    pub affected_batch_ids: Vec<Uuid>,
    pub remediation_status: RemediationStatus,
    pub reported_by: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportType {
    Internal,
    Customer,
    Regulatory,
    Recall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Seed,
    Plant,
    Bag,
    Zone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContaminationType {
    Bacterial(BacterialContamination),
    Fungal(FungalContamination),
    Pest(PestContamination),
    Chemical(ChemicalContamination),
    Physical(PhysicalContamination),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BacterialContamination {
    Salmonella,
    EColi,
    Listeria,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FungalContamination {
    Mold,
    Mildew,
    Rust,
    Blight,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PestContamination {
    Insects,
    Rodents,
    Birds,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChemicalContamination {
    Pesticide,
    Herbicide,
    HeavyMetals,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicalContamination {
    Foreign(String),
    Damage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemediationStatus {
    Identified,
    UnderInvestigation,
    Contained,
    Remediated,
    Monitoring,
    Closed,
}

/// USDA/FDA recall tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallAlert {
    pub id: Uuid,
    pub source_agency: RegulatoryAgency,
    pub recall_number: String,
    pub product_description: String,
    pub reason_for_recall: String,
    pub company_name: String,
    pub recall_date: DateTime<Utc>,
    pub distribution_area: String,
    pub affected_species: Vec<String>,
    pub contamination_type: Option<String>,
    pub health_hazard_level: HealthHazardLevel,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegulatoryAgency {
    Usda,
    Fda,
    Who,
    Cpsc,
    Epa,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthHazardLevel {
    ClassI,   // Serious adverse health consequences or death
    ClassII,  // Temporary or reversible adverse health consequences
    ClassIII, // Not likely to cause adverse health consequences
}

/// Food safety compliance checklist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyChecklist {
    pub id: Uuid,
    pub checklist_type: ChecklistType,
    pub performed_at: DateTime<Utc>,
    pub performed_by: String,
    pub items: Vec<ChecklistItem>,
    pub passed: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChecklistType {
    BagCleaning,
    SeedInspection,
    PlantHealth,
    GreenhouseHygiene,
    ShipmentPrep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub item_number: u32,
    pub description: String,
    pub checked: bool,
    pub compliant: bool,
    pub notes: Option<String>,
}

