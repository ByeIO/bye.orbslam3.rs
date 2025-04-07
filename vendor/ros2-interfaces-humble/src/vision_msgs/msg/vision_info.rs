use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisionInfo {
    pub header: crate::std_msgs::msg::Header,
    pub method: ::std::string::String,
    pub database_location: ::std::string::String,
    pub database_version: i32,
}

impl Default for VisionInfo {
    fn default() -> Self {
        VisionInfo {
            header: crate::std_msgs::msg::Header::default(),
            method: ::std::string::String::new(),
            database_location: ::std::string::String::new(),
            database_version: 0,
        }
    }
}

impl ros2_client::Message for VisionInfo {}
