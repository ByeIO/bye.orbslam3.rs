use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lateral {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub control_time: crate::builtin_interfaces::msg::Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
    pub is_defined_steering_tire_rotation_rate: bool,
}

impl Default for Lateral {
    fn default() -> Self {
        Lateral {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            control_time: crate::builtin_interfaces::msg::Time::default(),
            steering_tire_angle: 0.0,
            steering_tire_rotation_rate: 0.0,
            is_defined_steering_tire_rotation_rate: false,
        }
    }
}

impl ros2_client::Message for Lateral {}
