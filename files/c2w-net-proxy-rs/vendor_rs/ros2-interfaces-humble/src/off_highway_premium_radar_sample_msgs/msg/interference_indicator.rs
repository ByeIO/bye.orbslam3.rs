use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterferenceIndicator {
    pub fov_reduction_due_to_interfence: f32,
    pub interference_indicator: u8,
}

impl InterferenceIndicator {
    pub const INVALID_INTERFERENCE: u8 = 0;
    pub const VALID_NO_INTERFERENCE: u8 = 1;
    pub const VALID_INTERFERENCE: u8 = 2;
}

impl Default for InterferenceIndicator {
    fn default() -> Self {
        InterferenceIndicator {
            fov_reduction_due_to_interfence: 0.0,
            interference_indicator: 0,
        }
    }
}

impl ros2_client::Message for InterferenceIndicator {}
