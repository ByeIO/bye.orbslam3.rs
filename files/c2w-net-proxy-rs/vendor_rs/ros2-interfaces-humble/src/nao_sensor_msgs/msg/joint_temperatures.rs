use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointTemperatures {
    pub temperatures: [f32; 25],
}

impl Default for JointTemperatures {
    fn default() -> Self {
        JointTemperatures {
            temperatures: [0.0; 25],
        }
    }
}

impl ros2_client::Message for JointTemperatures {}
