use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarrSoln {
    pub status: u8,
}

impl CarrSoln {
    pub const CARRIER_SOLUTION_NO_CARRIER_RANGE_SOLUTION: u8 = 0;
    pub const CARRIER_SOLUTION_PHASE_WITH_FLOATING_AMBIGUITIES: u8 = 1;
    pub const CARRIER_SOLUTION_PHASE_WITH_FIXED_AMBIGUITIES: u8 = 2;
}

impl Default for CarrSoln {
    fn default() -> Self {
        CarrSoln {
            status: 0,
        }
    }
}

impl ros2_client::Message for CarrSoln {}
