use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintEvalResult {
    pub result: bool,
    pub distance: f64,
}

impl Default for ConstraintEvalResult {
    fn default() -> Self {
        ConstraintEvalResult {
            result: false,
            distance: 0.0,
        }
    }
}

impl ros2_client::Message for ConstraintEvalResult {}
