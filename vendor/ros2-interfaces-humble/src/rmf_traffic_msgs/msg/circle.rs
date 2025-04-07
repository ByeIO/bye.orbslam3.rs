use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub radius: f64,
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            radius: 0.0,
        }
    }
}

impl ros2_client::Message for Circle {}
