use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fans {
    pub fans: Vec<u8>,
}

impl Fans {
    pub const R100_EQUIPMENT_BAY_INTAKE: u8 = 0;
    pub const R100_EQUIPMENT_BAY_EXHAUST: u8 = 1;
    pub const R100_CHARGER_BAY_INTAKE: u8 = 2;
    pub const R100_CHARGER_BAY_EXHAUST: u8 = 3;
    pub const R100_USER_BAY_INTAKE: u8 = 4;
    pub const R100_USER_BAY_EXHAUST: u8 = 5;
    pub const FAN_OFF: u8 = 0;
    pub const FAN_ON_HIGH: u8 = 1;
    pub const FAN_ON_LOW: u8 = 2;
}

impl Default for Fans {
    fn default() -> Self {
        Fans {
            fans: Vec::new(),
        }
    }
}

impl ros2_client::Message for Fans {}
