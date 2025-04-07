use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorsLinearVelocity {
    pub velocity: Vec<f64>,
}

impl Default for ActuatorsLinearVelocity {
    fn default() -> Self {
        ActuatorsLinearVelocity {
            velocity: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActuatorsLinearVelocity {}
