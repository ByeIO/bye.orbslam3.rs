use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacle {
    pub id: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub polygon: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for Obstacle {
    fn default() -> Self {
        Obstacle {
            id: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            polygon: Vec::new(),
        }
    }
}

impl ros2_client::Message for Obstacle {}
