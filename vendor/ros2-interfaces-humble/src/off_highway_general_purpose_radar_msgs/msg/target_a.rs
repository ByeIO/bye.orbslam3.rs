use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetA {
    pub can_id: u32,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: u8,
    pub radial_distance: f64,
    pub radial_velocity: f64,
    pub reflected_power: f64,
    pub azimuth_angle: f64,
    pub measured: bool,
}

impl Default for TargetA {
    fn default() -> Self {
        TargetA {
            can_id: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: 0,
            radial_distance: 0.0,
            radial_velocity: 0.0,
            reflected_power: 0.0,
            azimuth_angle: 0.0,
            measured: false,
        }
    }
}

impl ros2_client::Message for TargetA {}
