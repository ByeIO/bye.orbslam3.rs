use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedTag {
    pub header: crate::std_msgs::msg::Header,
    pub tag: crate::rc_reason_msgs::msg::Tag,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub instance_id: ::std::string::String,
}

impl Default for DetectedTag {
    fn default() -> Self {
        DetectedTag {
            header: crate::std_msgs::msg::Header::default(),
            tag: crate::rc_reason_msgs::msg::Tag::default(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            instance_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DetectedTag {}
