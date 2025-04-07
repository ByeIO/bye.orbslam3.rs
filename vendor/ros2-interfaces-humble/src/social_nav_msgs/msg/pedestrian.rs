use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pedestrian {
    pub identifier: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose2D,
    pub velocity: crate::nav_2d_msgs::msg::Twist2D,
}

impl Default for Pedestrian {
    fn default() -> Self {
        Pedestrian {
            identifier: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose2D::default(),
            velocity: crate::nav_2d_msgs::msg::Twist2D::default(),
        }
    }
}

impl ros2_client::Message for Pedestrian {}
