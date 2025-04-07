use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2AudioMessage {
    pub header: crate::std_msgs::msg::Header,
    pub chunk_size: u32,
    pub channels: u8,
    pub sample_rate: u32,
    pub encoding: ::std::string::String,
    pub is_bigendian: u8,
    pub bitrate: u32,
    pub coding_format: ::std::string::String,
    pub step: u32,
    pub data: Vec<u8>,
}

impl Default for ROS2AudioMessage {
    fn default() -> Self {
        ROS2AudioMessage {
            header: crate::std_msgs::msg::Header::default(),
            chunk_size: 0,
            channels: 0,
            sample_rate: 0,
            encoding: ::std::string::String::new(),
            is_bigendian: 0,
            bitrate: 0,
            coding_format: ::std::string::String::new(),
            step: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for ROS2AudioMessage {}
