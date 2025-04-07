use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValueArray {
    pub header: crate::std_msgs::msg::Header,
    pub items: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for KeyValueArray {
    fn default() -> Self {
        KeyValueArray {
            header: crate::std_msgs::msg::Header::default(),
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for KeyValueArray {}
