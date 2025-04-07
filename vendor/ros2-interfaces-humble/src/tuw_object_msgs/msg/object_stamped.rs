use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectStamped {
    pub header: crate::std_msgs::msg::Header,
    pub object: crate::tuw_object_msgs::msg::Object,
}

impl Default for ObjectStamped {
    fn default() -> Self {
        ObjectStamped {
            header: crate::std_msgs::msg::Header::default(),
            object: crate::tuw_object_msgs::msg::Object::default(),
        }
    }
}

impl ros2_client::Message for ObjectStamped {}
