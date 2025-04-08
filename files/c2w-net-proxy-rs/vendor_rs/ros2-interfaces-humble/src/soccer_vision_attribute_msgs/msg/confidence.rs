use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Confidence {
    pub confidence: f32, // default: -1.0
}

impl Confidence {
    pub const CONFIDENCE_UNKNOWN: f32 = -1.0;
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence {
            confidence: -1.0,
        }
    }
}

impl ros2_client::Message for Confidence {}
