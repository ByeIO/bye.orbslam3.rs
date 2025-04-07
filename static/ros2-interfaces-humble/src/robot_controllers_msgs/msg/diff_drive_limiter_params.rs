use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffDriveLimiterParams {
    pub max_linear_velocity: f64,
    pub max_linear_acceleration: f64,
    pub max_angular_velocity: f64,
    pub max_angular_acceleration: f64,
    pub max_wheel_velocity: f64,
    pub track_width: f64,
    pub angular_velocity_limits_linear_velocity: bool,
    pub scale_to_wheel_velocity_limits: bool,
}

impl Default for DiffDriveLimiterParams {
    fn default() -> Self {
        DiffDriveLimiterParams {
            max_linear_velocity: 0.0,
            max_linear_acceleration: 0.0,
            max_angular_velocity: 0.0,
            max_angular_acceleration: 0.0,
            max_wheel_velocity: 0.0,
            track_width: 0.0,
            angular_velocity_limits_linear_velocity: false,
            scale_to_wheel_velocity_limits: false,
        }
    }
}

impl ros2_client::Message for DiffDriveLimiterParams {}
