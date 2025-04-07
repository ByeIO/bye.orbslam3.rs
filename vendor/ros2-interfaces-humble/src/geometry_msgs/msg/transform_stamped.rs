use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformStamped {
    pub header: crate::std_msgs::msg::Header,
    pub child_frame_id: ::std::string::String,
    pub transform: crate::geometry_msgs::msg::Transform,
}

impl Default for TransformStamped {
    fn default() -> Self {
        TransformStamped {
            header: crate::std_msgs::msg::Header::default(),
            child_frame_id: ::std::string::String::new(),
            transform: crate::geometry_msgs::msg::Transform::default(),
        }
    }
}

impl ros2_client::Message for TransformStamped {}
