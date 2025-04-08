use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveFeedback {
    pub current: f32,
    pub duty_cycle: f32,
    pub bridge_temperature: f32,
    pub motor_temperature: f32,
    pub measured_velocity: f32,
    pub measured_travel: f32,
    pub driver_fault: bool,
}

impl Default for DriveFeedback {
    fn default() -> Self {
        DriveFeedback {
            current: 0.0,
            duty_cycle: 0.0,
            bridge_temperature: 0.0,
            motor_temperature: 0.0,
            measured_velocity: 0.0,
            measured_travel: 0.0,
            driver_fault: false,
        }
    }
}

impl ros2_client::Message for DriveFeedback {}
