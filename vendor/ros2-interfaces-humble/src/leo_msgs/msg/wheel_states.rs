use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelStates {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub position: [f32; 4],
    pub velocity: [f32; 4],
    pub torque: [f32; 4],
    pub pwm_duty_cycle: [f32; 4],
}

impl Default for WheelStates {
    fn default() -> Self {
        WheelStates {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            position: [0.0; 4],
            velocity: [0.0; 4],
            torque: [0.0; 4],
            pwm_duty_cycle: [0.0; 4],
        }
    }
}

impl ros2_client::Message for WheelStates {}
