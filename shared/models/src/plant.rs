use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Plant processing information for consumption or curing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantProcessingGuide {
    pub id: Uuid,
    pub species: String,
    pub variety: Option<String>,
    pub is_edible: bool,
    pub edible_parts: Vec<EdiblePart>,
    pub processing_methods: Vec<ProcessingMethod>,
    pub safety_warnings: Vec<String>,
    pub nutritional_info: Option<NutritionalInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiblePart {
    pub part_name: String, // "fruit", "leaves", "seeds", "roots", etc.
    pub edible: bool,
    pub preparation_required: bool,
    pub toxicity_warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMethod {
    pub method_type: ProcessingType,
    pub name: String,
    pub description: String,
    pub duration_hours: Option<u32>,
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    pub steps: Vec<ProcessingStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessingType {
    /// Immediate consumption
    Eating,
    /// Drying and curing process
    Curing,
    /// Cooking preparation
    Cooking,
    /// Fermentation
    Fermentation,
    /// Preservation
    Preservation,
    /// Extraction
    Extraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStep {
    pub step_number: u32,
    pub instruction: String,
    pub duration_minutes: Option<u32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalInfo {
    pub serving_size_grams: u32,
    pub calories: Option<u32>,
    pub protein_grams: Option<f32>,
    pub carbs_grams: Option<f32>,
    pub fat_grams: Option<f32>,
    pub fiber_grams: Option<f32>,
    pub vitamins: Vec<String>,
}

/// Curing protocol specifically for crops that require curing (like cannabis)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuringProtocol {
    pub id: Uuid,
    pub plant_type: String,
    pub protocol_name: String,
    pub phases: Vec<CuringPhase>,
    pub total_duration_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuringPhase {
    pub phase_number: u32,
    pub name: String,
    pub duration_days: u32,
    pub temperature_range_celsius: TemperatureRange,
    pub humidity_range_percent: HumidityRange,
    pub air_circulation: bool,
    pub light_exposure: LightExposure,
    pub instructions: String,
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
    None,
    Minimal,
    Indirect,
    Direct,
}

/// Recipe suggestions for edible crops
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub plant_species: String,
    pub cuisine_type: Option<String>,
    pub difficulty: DifficultyLevel,
    pub prep_time_minutes: u32,
    pub cook_time_minutes: u32,
    pub servings: u32,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<RecipeStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub item: String,
    pub amount: String,
    pub preparation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub step_number: u32,
    pub instruction: String,
}

