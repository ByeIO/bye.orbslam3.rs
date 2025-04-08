use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extrinsics {
    pub rotation: [f64; 9],
    pub translation: [f64; 3],
}

impl Default for Extrinsics {
    fn default() -> Self {
        Extrinsics {
            rotation: [0.0; 9],
            translation: [0.0; 3],
        }
    }
}

impl ros2_client::Message for Extrinsics {}
