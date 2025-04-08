use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gphdt {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub heading: f64,
    pub t: ::std::string::String,
}

impl Default for Gphdt {
    fn default() -> Self {
        Gphdt {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            heading: 0.0,
            t: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Gphdt {}
