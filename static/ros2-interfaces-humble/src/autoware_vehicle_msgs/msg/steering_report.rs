use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteeringReport {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub steering_tire_angle: f32,
}

impl Default for SteeringReport {
    fn default() -> Self {
        SteeringReport {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            steering_tire_angle: 0.0,
        }
    }
}

impl ros2_client::Message for SteeringReport {}
