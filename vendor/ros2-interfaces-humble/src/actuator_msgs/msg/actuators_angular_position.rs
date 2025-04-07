use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorsAngularPosition {
    pub position: Vec<f64>,
}

impl Default for ActuatorsAngularPosition {
    fn default() -> Self {
        ActuatorsAngularPosition {
            position: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActuatorsAngularPosition {}
