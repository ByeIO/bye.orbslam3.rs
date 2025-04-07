use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointCurrents {
    pub currents: [f32; 25],
}

impl Default for JointCurrents {
    fn default() -> Self {
        JointCurrents {
            currents: [0.0; 25],
        }
    }
}

impl ros2_client::Message for JointCurrents {}
