use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointStiffnesses {
    pub stiffnesses: [f32; 25],
}

impl Default for JointStiffnesses {
    fn default() -> Self {
        JointStiffnesses {
            stiffnesses: [0.0; 25],
        }
    }
}

impl ros2_client::Message for JointStiffnesses {}
