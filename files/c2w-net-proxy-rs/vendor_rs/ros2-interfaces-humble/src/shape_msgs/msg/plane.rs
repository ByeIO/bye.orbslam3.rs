use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub coef: [f64; 4],
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            coef: [0.0; 4],
        }
    }
}

impl ros2_client::Message for Plane {}
