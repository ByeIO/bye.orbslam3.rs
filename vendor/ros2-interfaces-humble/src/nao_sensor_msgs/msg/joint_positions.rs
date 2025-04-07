use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointPositions {
    pub positions: [f32; 25],
}

impl Default for JointPositions {
    fn default() -> Self {
        JointPositions {
            positions: [0.0; 25],
        }
    }
}

impl ros2_client::Message for JointPositions {}
