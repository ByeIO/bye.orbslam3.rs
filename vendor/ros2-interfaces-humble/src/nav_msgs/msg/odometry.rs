use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Odometry {
    pub header: crate::std_msgs::msg::Header,
    pub child_frame_id: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl Default for Odometry {
    fn default() -> Self {
        Odometry {
            header: crate::std_msgs::msg::Header::default(),
            child_frame_id: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for Odometry {}
