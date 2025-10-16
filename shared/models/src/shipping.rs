use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::subscription::Address;

/// Multi-point shipping configuration for bag collection and return
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentCycle {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub subscription_id: Uuid,
    pub legs: Vec<ShipmentLeg>,
    pub status: ShipmentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentLeg {
    pub leg_number: u32,
    pub leg_type: ShipmentLegType,
    pub from_address: Address,
    pub to_address: Address,
    pub shipstation_label_id: Option<String>,
    pub tracking_number: Option<String>,
    pub status: LegStatus,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipmentLegType {
    /// Customer sends bags to facility (Leg 1)
    CustomerToFacility,
    /// Facility sends cleaned bags back to customer (Final leg with return label)
    FacilityToCustomer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipmentStatus {
    Pending,
    InTransit,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LegStatus {
    Pending,
    LabelCreated,
    Shipped,
    InTransit,
    Delivered,
    Failed,
}

/// Bag folding instructions for packaging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BagPackagingInstructions {
    pub steps: Vec<PackagingStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagingStep {
    pub step_number: u32,
    pub instruction: String,
    pub image_url: Option<String>,
}

impl BagPackagingInstructions {
    pub fn trapezoid_butterfly_method() -> Self {
        Self {
            steps: vec![
                PackagingStep {
                    step_number: 1,
                    instruction: "Lay the bag flat on a clean surface with handles facing up.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 2,
                    instruction: "Fold a trapezoid shape on the left side, creating the first wing.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 3,
                    instruction: "Fold a matching trapezoid on the right side, forming the second wing of the butterfly.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 4,
                    instruction: "Twist both handles inward toward the center of the butterfly shape.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 5,
                    instruction: "Place the shipping label over the wing shape, covering the center.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 6,
                    instruction: "Cover the label completely with a perforated sticker that has a cut line.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 7,
                    instruction: "The sticker will be cut with rollers to create an easy-tear perforated shipping label.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 8,
                    instruction: "Bend each halved sticker-covered half of the bag.".to_string(),
                    image_url: None,
                },
                PackagingStep {
                    step_number: 9,
                    instruction: "Wrap tape completely around each half, creating a compact cube shape resembling a Rubik's cube.".to_string(),
                    image_url: None,
                },
            ],
        }
    }
}

/// ShipStation API integration models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStationLabel {
    pub shipment_id: String,
    pub label_data: String, // Base64 encoded label
    pub tracking_number: String,
    pub carrier_code: String,
    pub service_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStationCreateLabelRequest {
    pub order_id: String,
    pub carrier_code: String,
    pub service_code: String,
    pub confirmation: String,
    pub ship_date: String,
    pub weight: Weight,
    pub dimensions: Option<Dimensions>,
    pub ship_from: ShipStationAddress,
    pub ship_to: ShipStationAddress,
    pub is_return_label: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStationAddress {
    pub name: String,
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    pub value: f32,
    pub units: String, // "ounces" or "grams"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
    pub units: String, // "inches" or "centimeters"
}

