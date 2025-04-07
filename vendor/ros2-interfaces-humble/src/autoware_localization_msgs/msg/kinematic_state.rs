use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinematicState {
    pub header: crate::std_msgs::msg::Header,
    pub child_frame_id: ::std::string::String,
    pub pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance,
    pub twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance,
    pub accel_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance,
}

impl Default for KinematicState {
    fn default() -> Self {
        KinematicState {
            header: crate::std_msgs::msg::Header::default(),
            child_frame_id: ::std::string::String::new(),
            pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            accel_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for KinematicState {}
