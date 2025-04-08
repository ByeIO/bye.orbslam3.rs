use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotorSetpoint {
    pub duty_cycle: f32,
}

impl Default for MotorSetpoint {
    fn default() -> Self {
        MotorSetpoint {
            duty_cycle: 0.0,
        }
    }
}

impl ros2_client::Message for MotorSetpoint {}
