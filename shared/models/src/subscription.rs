use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Subscription tiers for the Seed Box Bag Box service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionTier {
    /// $8/month - Requires customer to provide bags
    BringYourOwnBags,
    /// $15/month - Standard service with random bag sampling
    Standard,
    /// $19/month - Premium service with only your own bags returned
    Premium,
}

impl SubscriptionTier {
    pub fn price_cents(&self) -> u32 {
        match self {
            Self::BringYourOwnBags => 800,
            Self::Standard => 1500,
            Self::Premium => 1900,
        }
    }

    pub fn receives_random_bags(&self) -> bool {
        matches!(self, Self::Standard)
    }

    pub fn requires_bags(&self) -> bool {
        matches!(self, Self::BringYourOwnBags)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tier: SubscriptionTier,
    pub status: SubscriptionStatus,
    pub cratejoy_subscription_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub next_billing_date: DateTime<Utc>,
    pub bags_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Cancelled,
    PendingBags, // Waiting for customer to send bags
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub shipping_address: Address,
    pub created_at: DateTime<Utc>,
    pub cratejoy_customer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

