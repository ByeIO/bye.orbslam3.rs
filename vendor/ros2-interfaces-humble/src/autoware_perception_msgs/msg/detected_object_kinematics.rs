use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedObjectKinematics {
    pub pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance,
    pub has_position_covariance: bool,
    pub orientation_availability: u8,
    pub twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance,
    pub has_twist: bool,
    pub has_twist_covariance: bool,
}

impl DetectedObjectKinematics {
    pub const UNAVAILABLE: u8 = 0;
    pub const SIGN_UNKNOWN: u8 = 1;
    pub const AVAILABLE: u8 = 2;
}

impl Default for DetectedObjectKinematics {
    fn default() -> Self {
        DetectedObjectKinematics {
            pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            has_position_covariance: false,
            orientation_availability: 0,
            twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            has_twist: false,
            has_twist_covariance: false,
        }
    }
}

impl ros2_client::Message for DetectedObjectKinematics {}
