use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedObjectKinematics {
    pub pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance,
    pub twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance,
    pub acceleration_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance,
    pub orientation_availability: u8,
    pub is_stationary: bool,
}

impl TrackedObjectKinematics {
    pub const UNAVAILABLE: u8 = 0;
    pub const SIGN_UNKNOWN: u8 = 1;
    pub const AVAILABLE: u8 = 2;
}

impl Default for TrackedObjectKinematics {
    fn default() -> Self {
        TrackedObjectKinematics {
            pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            acceleration_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance::default(),
            orientation_availability: 0,
            is_stationary: false,
        }
    }
}

impl ros2_client::Message for TrackedObjectKinematics {}
