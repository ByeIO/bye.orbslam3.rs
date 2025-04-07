use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub position: crate::geometry_msgs::msg::Point,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            position: crate::geometry_msgs::msg::Point::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
        }
    }
}

impl ros2_client::Message for Pose {}
