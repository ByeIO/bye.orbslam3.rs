use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseWithID {
    pub id: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for PoseWithID {
    fn default() -> Self {
        PoseWithID {
            id: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for PoseWithID {}
