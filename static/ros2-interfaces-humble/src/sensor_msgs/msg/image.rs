use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub encoding: ::std::string::String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Vec<u8>,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            header: crate::std_msgs::msg::Header::default(),
            height: 0,
            width: 0,
            encoding: ::std::string::String::new(),
            is_bigendian: 0,
            step: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Image {}
