use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringArrayStamped {
    pub header: crate::std_msgs::msg::Header,
    pub strings: Vec<::std::string::String>,
}

impl Default for StringArrayStamped {
    fn default() -> Self {
        StringArrayStamped {
            header: crate::std_msgs::msg::Header::default(),
            strings: Vec::new(),
        }
    }
}

impl ros2_client::Message for StringArrayStamped {}
