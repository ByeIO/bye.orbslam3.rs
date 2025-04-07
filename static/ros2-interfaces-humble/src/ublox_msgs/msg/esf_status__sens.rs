use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfSTATUS_Sens {
    pub sensStatus1: u8,
    pub sensStatus2: u8,
    pub freq: u8,
    pub faults: u8,
}

impl Default for EsfSTATUS_Sens {
    fn default() -> Self {
        EsfSTATUS_Sens {
            sensStatus1: 0,
            sensStatus2: 0,
            freq: 0,
            faults: 0,
        }
    }
}

impl ros2_client::Message for EsfSTATUS_Sens {}
