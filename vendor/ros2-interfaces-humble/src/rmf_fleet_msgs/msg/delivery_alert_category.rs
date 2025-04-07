use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryAlertCategory {
    pub value: u32,
}

impl DeliveryAlertCategory {
    pub const MISSING: u32 = 0;
    pub const WRONG: u32 = 1;
    pub const OBSTRUCTED: u32 = 2;
    pub const CANCELLED: u32 = 3;
}

impl Default for DeliveryAlertCategory {
    fn default() -> Self {
        DeliveryAlertCategory {
            value: 0,
        }
    }
}

impl ros2_client::Message for DeliveryAlertCategory {}
