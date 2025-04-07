use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalPower {
    pub source: u8,
    pub state: u8,
}

impl ExternalPower {
    pub const PWR_3_3V1A: u8 = 0;
    pub const PWR_5V1A: u8 = 1;
    pub const PWR_12V5A: u8 = 2;
    pub const PWR_12V1_5A: u8 = 3;
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;
}

impl Default for ExternalPower {
    fn default() -> Self {
        ExternalPower {
            source: 0,
            state: 0,
        }
    }
}

impl ros2_client::Message for ExternalPower {}
