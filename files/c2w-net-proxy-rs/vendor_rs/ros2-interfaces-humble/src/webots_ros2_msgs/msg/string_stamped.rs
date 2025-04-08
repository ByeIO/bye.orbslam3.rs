use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: ::std::string::String,
}

impl Default for StringStamped {
    fn default() -> Self {
        StringStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StringStamped {}
