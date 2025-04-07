use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointLimits {
    pub joint_name: ::std::string::String,
    pub has_position_limits: bool,
    pub min_position: f64,
    pub max_position: f64,
    pub has_velocity_limits: bool,
    pub max_velocity: f64,
    pub has_acceleration_limits: bool,
    pub max_acceleration: f64,
    pub has_jerk_limits: bool,
    pub max_jerk: f64,
}

impl Default for JointLimits {
    fn default() -> Self {
        JointLimits {
            joint_name: ::std::string::String::new(),
            has_position_limits: false,
            min_position: 0.0,
            max_position: 0.0,
            has_velocity_limits: false,
            max_velocity: 0.0,
            has_acceleration_limits: false,
            max_acceleration: 0.0,
            has_jerk_limits: false,
            max_jerk: 0.0,
        }
    }
}

impl ros2_client::Message for JointLimits {}
