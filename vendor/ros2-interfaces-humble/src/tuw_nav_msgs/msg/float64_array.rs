use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float64Array {
    pub val: Vec<f64>,
}

impl Default for Float64Array {
    fn default() -> Self {
        Float64Array {
            val: Vec::new(),
        }
    }
}

impl ros2_client::Message for Float64Array {}
