use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeValue {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

impl Default for RangeValue {
    fn default() -> Self {
        RangeValue {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
        }
    }
}

impl ros2_client::Message for RangeValue {}
