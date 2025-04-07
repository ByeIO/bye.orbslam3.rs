use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Longitudinal {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub control_time: crate::builtin_interfaces::msg::Time,
    pub velocity: f32,
    pub acceleration: f32,
    pub jerk: f32,
    pub is_defined_acceleration: bool,
    pub is_defined_jerk: bool,
}

impl Default for Longitudinal {
    fn default() -> Self {
        Longitudinal {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            control_time: crate::builtin_interfaces::msg::Time::default(),
            velocity: 0.0,
            acceleration: 0.0,
            jerk: 0.0,
            is_defined_acceleration: false,
            is_defined_jerk: false,
        }
    }
}

impl ros2_client::Message for Longitudinal {}
