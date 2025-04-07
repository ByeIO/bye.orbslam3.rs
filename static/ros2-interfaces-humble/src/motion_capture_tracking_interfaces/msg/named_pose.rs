use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedPose {
    pub name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for NamedPose {
    fn default() -> Self {
        NamedPose {
            name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for NamedPose {}
