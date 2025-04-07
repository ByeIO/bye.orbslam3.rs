use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorsAngularVelocity {
    pub velocity: Vec<f64>,
}

impl Default for ActuatorsAngularVelocity {
    fn default() -> Self {
        ActuatorsAngularVelocity {
            velocity: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActuatorsAngularVelocity {}
