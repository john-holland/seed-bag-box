use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Germination tracking for seeds that will be sent back as sprouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GerminationRecord {
    pub id: Uuid,
    pub seed_id: Uuid,
    pub plant_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub species: String,
    pub variety: Option<String>,
    
    /// Germination phase tracking
    pub germination_phase: GerminationPhase,
    pub started_at: DateTime<Utc>,
    
    /// Botanical stage timestamps
    pub imbibition_started_at: DateTime<Utc>,
    pub radicle_emerged_at: Option<DateTime<Utc>>,
    pub shoot_emerged_at: Option<DateTime<Utc>>,
    pub cotyledon_expanded_at: Option<DateTime<Utc>>,
    pub true_leaf_emerged_at: Option<DateTime<Utc>>, // Becomes "true plant" at this point
    pub photosynthesis_started_at: Option<DateTime<Utc>>,
    pub ready_for_shipment_at: Option<DateTime<Utc>>,
    
    /// Growing conditions
    pub growing_medium: GrowingMedium,
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    pub light_hours_per_day: Option<f32>,
    
    /// Health and quality
    pub germination_success: bool,
    pub health_status: SproutHealthStatus,
    
    /// Growth measurements
    pub root_length_mm: Option<f32>,
    pub shoot_length_mm: Option<f32>,
    pub cotyledon_count: Option<u32>, // Usually 2, but can vary by species
    pub true_leaf_count: Option<u32>, // Count of true leaves (not cotyledons)
    pub total_leaf_count: Option<u32>, // Cotyledons + true leaves
    
    /// True plant indicators
    pub is_true_plant: bool, // True when true leaves have emerged
    pub is_autotrophic: bool, // True when photosynthesizing independently
    
    /// Edible parts indicators
    pub has_edible_fruit_potential: Option<bool>, // Whether this species bears edible fruit
    pub has_edible_leaves_potential: Option<bool>, // Whether this species has edible leaves
    pub has_edible_stalks_potential: Option<bool>, // Whether this species has edible stalks/stems
    
    /// Shipment information
    pub shipment_type: ShipmentType,
    pub estimated_ship_date: Option<DateTime<Utc>>,
    pub actual_ship_date: Option<DateTime<Utc>>,
    pub customer_instructions: Option<String>,
    
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GerminationPhase {
    /// Phase 1: Seed absorbs water and begins to swell
    Imbibition,
    /// Phase 2: First root (radicle) emerges from seed coat to anchor and absorb water
    RadicleEmergence,
    /// Phase 3: Shoot emerges, stem grows upwards with cotyledons still folded
    ShootEmergence,
    /// Phase 4: First two seed leaves (cotyledons) unfold - not true leaves yet
    CotyledonExpansion,
    /// Phase 5: First true leaves develop with characteristic species shape - "true plant" stage
    TrueLeafEmergence,
    /// Phase 6: True leaves performing photosynthesis, plant is autotrophic
    Photosynthesis,
    /// Phase 7: Continued growth with multiple true leaves and strong roots
    ContinuedGrowth,
    /// Ready for customer shipment
    ReadyForShipment,
    /// Shipped to customer
    Shipped,
    /// Customer confirmed receipt
    Delivered,
    /// Failed to germinate at any stage
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GrowingMedium {
    Soil,
    Rockwool,
    Coco,
    Peat,
    Hydroponic,
    Paper,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SproutHealthStatus {
    Excellent,
    Good,
    Fair,
    Weak,
    Diseased,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipmentType {
    /// Sprout with roots in growing medium
    LiveSprout,
    /// Bare root sprout (medium removed)
    BareRoot,
    /// In small pot/container
    Potted,
    /// Microgreens (cut, for eating)
    Microgreens,
}

/// Species-specific germination guide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GerminationGuide {
    pub species: String,
    pub variety: Option<String>,
    
    /// Timing information for each stage
    pub imbibition_days: u32, // Days for seed to absorb water and swell (1-3 typically)
    pub radicle_emergence_days_min: u32,
    pub radicle_emergence_days_max: u32,
    pub shoot_emergence_days_min: u32,
    pub shoot_emergence_days_max: u32,
    pub cotyledon_expansion_days_min: u32,
    pub cotyledon_expansion_days_max: u32,
    pub true_leaf_emergence_days_min: u32, // When it becomes a "true plant"
    pub true_leaf_emergence_days_max: u32,
    pub photosynthesis_days_min: u32, // When plant becomes autotrophic
    pub photosynthesis_days_max: u32,
    pub ready_to_ship_days: u32, // Days after imbibition started
    
    /// Optimal conditions
    pub optimal_temperature_celsius: TemperatureRange,
    pub optimal_humidity_percent: HumidityRange,
    pub light_requirement: LightRequirement,
    pub preferred_medium: Vec<GrowingMedium>,
    
    /// Shipping readiness criteria
    pub min_root_length_mm: f32,
    pub min_shoot_length_mm: f32,
    pub min_true_leaf_count: u32, // Minimum true leaves (not cotyledons)
    pub must_be_true_plant: bool, // Require true leaves before shipping
    pub must_be_autotrophic: bool, // Require photosynthesis before shipping
    
    /// Special instructions
    pub pre_soak_required: bool,
    pub pre_soak_hours: Option<u32>,
    pub scarification_required: bool,
    pub stratification_required: bool,
    pub stratification_days: Option<u32>,
    
    /// Care instructions for customer
    pub planting_depth_mm: f32,
    pub spacing_cm: f32,
    pub days_to_maturity: u32,
    pub customer_care_instructions: String,
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
pub enum LightRequirement {
    /// No light needed, darkness preferred
    Dark,
    /// Low light sufficient
    Low,
    /// Medium indirect light
    Medium,
    /// High direct light
    High,
    /// Specific photoperiod required
    Photoperiod { hours_per_day: f32 },
}

/// Daily observation log for germination tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GerminationObservation {
    pub id: Uuid,
    pub germination_record_id: Uuid,
    pub observed_at: DateTime<Utc>,
    pub observed_by: String,
    
    pub root_length_mm: Option<f32>,
    pub shoot_length_mm: Option<f32>,
    pub cotyledon_count: Option<u32>,
    pub true_leaf_count: Option<u32>,
    pub total_leaf_count: Option<u32>,
    pub cotyledon_color: Option<String>,
    pub true_leaf_color: Option<String>,
    pub health_status: SproutHealthStatus,
    
    /// Botanical observations
    pub radicle_visible: bool,
    pub shoot_visible: bool,
    pub cotyledons_expanded: bool,
    pub true_leaves_present: bool,
    pub appears_autotrophic: bool, // Leaves look healthy and photosynthesizing
    
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    
    pub issues_noted: Vec<String>,
    pub actions_taken: Vec<String>,
    pub notes: Option<String>,
    pub photo_url: Option<String>,
}

/// Package configuration for shipping live sprouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SproutShipmentPackage {
    pub id: Uuid,
    pub germination_records: Vec<Uuid>, // Multiple sprouts can ship together
    pub customer_id: Uuid,
    pub shipment_cycle_id: Option<Uuid>,
    
    pub package_type: PackageType,
    pub container_count: u32,
    pub total_sprouts: u32,
    
    /// Packaging materials
    pub moisture_retention: bool, // Wet paper towels, etc.
    pub temperature_control: bool, // Ice packs or heat packs
    pub ventilation: bool,
    
    /// Shipping constraints
    pub expedited_shipping_required: bool,
    pub ship_on_days: Vec<Weekday>, // e.g., only ship Mon-Wed to avoid weekend delays
    pub max_transit_days: u32,
    
    /// Customer communication
    pub care_instructions_included: bool,
    pub transplant_instructions_included: bool,
    pub species_info_card_included: bool,
    
    pub packed_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub expected_delivery: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageType {
    /// Small cardboard box with ventilation holes
    VentilatedBox,
    /// Plastic container with secure lid
    PlasticContainer,
    /// Padded envelope for microgreens
    PaddedEnvelope,
    /// Custom packaging
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Customer preferences for sprout delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSproutPreferences {
    pub customer_id: Uuid,
    
    /// What types of sprouts customer wants
    pub preferred_species: Vec<String>,
    pub excluded_species: Vec<String>,
    pub preferred_shipment_type: ShipmentType,
    
    /// Timing preferences
    pub delivery_frequency: DeliveryFrequency,
    pub preferred_delivery_days: Vec<Weekday>,
    
    /// Quantity preferences
    pub sprouts_per_shipment_min: u32,
    pub sprouts_per_shipment_max: u32,
    
    /// Growing experience level
    pub experience_level: ExperienceLevel,
    pub has_grow_lights: bool,
    pub has_greenhouse: bool,
    pub has_outdoor_space: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeliveryFrequency {
    Weekly,
    Biweekly,
    Monthly,
    Seasonal,
    AsReady, // Ship as soon as sprouts are ready
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExperienceLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl GerminationPhase {
    /// Get the next phase in the germination process
    pub fn next_phase(&self) -> Option<Self> {
        match self {
            Self::Imbibition => Some(Self::RadicleEmergence),
            Self::RadicleEmergence => Some(Self::ShootEmergence),
            Self::ShootEmergence => Some(Self::CotyledonExpansion),
            Self::CotyledonExpansion => Some(Self::TrueLeafEmergence),
            Self::TrueLeafEmergence => Some(Self::Photosynthesis),
            Self::Photosynthesis => Some(Self::ContinuedGrowth),
            Self::ContinuedGrowth => Some(Self::ReadyForShipment),
            Self::ReadyForShipment => Some(Self::Shipped),
            Self::Shipped => Some(Self::Delivered),
            Self::Delivered | Self::Failed => None,
        }
    }
    
    /// Check if sprout is ready for shipment
    pub fn is_shippable(&self) -> bool {
        matches!(self, Self::ReadyForShipment)
    }
    
    /// Check if plant has reached "true plant" stage (true leaves emerged)
    pub fn is_true_plant(&self) -> bool {
        matches!(
            self,
            Self::TrueLeafEmergence
                | Self::Photosynthesis
                | Self::ContinuedGrowth
                | Self::ReadyForShipment
                | Self::Shipped
                | Self::Delivered
        )
    }
    
    /// Check if plant is autotrophic (performing photosynthesis)
    pub fn is_autotrophic(&self) -> bool {
        matches!(
            self,
            Self::Photosynthesis
                | Self::ContinuedGrowth
                | Self::ReadyForShipment
                | Self::Shipped
                | Self::Delivered
        )
    }
    
    /// Get human-readable description of the phase
    pub fn description(&self) -> &str {
        match self {
            Self::Imbibition => "Seed absorbing water and swelling",
            Self::RadicleEmergence => "First root emerging from seed coat",
            Self::ShootEmergence => "Shoot emerging with folded cotyledons",
            Self::CotyledonExpansion => "Seed leaves (cotyledons) unfolding",
            Self::TrueLeafEmergence => "First true leaves developing - becoming a true plant",
            Self::Photosynthesis => "True leaves performing photosynthesis - plant is autotrophic",
            Self::ContinuedGrowth => "Growing strong with multiple true leaves",
            Self::ReadyForShipment => "Ready to ship to customer",
            Self::Shipped => "In transit to customer",
            Self::Delivered => "Delivered to customer",
            Self::Failed => "Germination failed",
        }
    }
}

impl GerminationRecord {
    /// Check if sprout meets shipment criteria based on guide
    pub fn meets_shipment_criteria(&self, guide: &GerminationGuide) -> bool {
        if !self.germination_success {
            return false;
        }
        
        // Check physical measurements
        let root_ok = self.root_length_mm
            .map(|len| len >= guide.min_root_length_mm)
            .unwrap_or(false);
        
        let shoot_ok = self.shoot_length_mm
            .map(|len| len >= guide.min_shoot_length_mm)
            .unwrap_or(false);
        
        let true_leaf_ok = self.true_leaf_count
            .map(|count| count >= guide.min_true_leaf_count)
            .unwrap_or(false);
        
        // Check botanical maturity
        let true_plant_ok = !guide.must_be_true_plant || self.is_true_plant;
        let autotrophic_ok = !guide.must_be_autotrophic || self.is_autotrophic;
        
        // Check health
        let health_ok = matches!(
            self.health_status,
            SproutHealthStatus::Excellent | SproutHealthStatus::Good
        );
        
        root_ok && shoot_ok && true_leaf_ok && true_plant_ok && autotrophic_ok && health_ok
    }
    
    /// Update the is_true_plant flag based on phase and true leaf count
    pub fn update_true_plant_status(&mut self) {
        self.is_true_plant = self.germination_phase.is_true_plant()
            && self.true_leaf_count.unwrap_or(0) > 0;
    }
    
    /// Update the is_autotrophic flag based on phase
    pub fn update_autotrophic_status(&mut self) {
        self.is_autotrophic = self.germination_phase.is_autotrophic()
            && self.is_true_plant;
    }
}

