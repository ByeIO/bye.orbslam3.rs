use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractiveMarkerPose {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub name: ::std::string::String,
}

impl Default for InteractiveMarkerPose {
    fn default() -> Self {
        InteractiveMarkerPose {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for InteractiveMarkerPose {}
