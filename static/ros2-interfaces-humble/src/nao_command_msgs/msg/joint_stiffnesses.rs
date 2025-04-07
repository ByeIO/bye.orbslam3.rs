use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointStiffnesses {
    pub indexes: Vec<u8>,
    pub stiffnesses: Vec<f32>,
}

impl Default for JointStiffnesses {
    fn default() -> Self {
        JointStiffnesses {
            indexes: Vec::new(),
            stiffnesses: Vec::new(),
        }
    }
}

impl ros2_client::Message for JointStiffnesses {}
