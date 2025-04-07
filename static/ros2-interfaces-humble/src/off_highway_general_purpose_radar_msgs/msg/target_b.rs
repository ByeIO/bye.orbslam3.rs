use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetB {
    pub can_id: u32,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: u8,
    pub azimuth_angle_std: f64,
    pub radial_velocity_std: f64,
    pub radial_distance_std: f64,
    pub exist_probability: f64,
    pub time_since_meas: f64,
}

impl Default for TargetB {
    fn default() -> Self {
        TargetB {
            can_id: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: 0,
            azimuth_angle_std: 0.0,
            radial_velocity_std: 0.0,
            radial_distance_std: 0.0,
            exist_probability: 0.0,
            time_since_meas: 0.0,
        }
    }
}

impl ros2_client::Message for TargetB {}
