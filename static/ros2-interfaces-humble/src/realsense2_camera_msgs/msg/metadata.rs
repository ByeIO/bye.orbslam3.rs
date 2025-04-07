use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub header: crate::std_msgs::msg::Header,
    pub json_data: ::std::string::String,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            header: crate::std_msgs::msg::Header::default(),
            json_data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Metadata {}
