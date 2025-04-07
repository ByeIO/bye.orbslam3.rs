use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfSTATUSSens {
    pub sens_status1: u8,
    pub sens_status2: u8,
    pub freq: u8,
    pub faults: u8,
}

impl Default for EsfSTATUSSens {
    fn default() -> Self {
        EsfSTATUSSens {
            sens_status1: 0,
            sens_status2: 0,
            freq: 0,
            faults: 0,
        }
    }
}

impl ros2_client::Message for EsfSTATUSSens {}
