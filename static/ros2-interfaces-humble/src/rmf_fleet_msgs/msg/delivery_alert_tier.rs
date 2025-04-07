use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryAlertTier {
    pub value: u32,
}

impl DeliveryAlertTier {
    pub const WARNING: u32 = 0;
    pub const ERROR: u32 = 1;
}

impl Default for DeliveryAlertTier {
    fn default() -> Self {
        DeliveryAlertTier {
            value: 0,
        }
    }
}

impl ros2_client::Message for DeliveryAlertTier {}
