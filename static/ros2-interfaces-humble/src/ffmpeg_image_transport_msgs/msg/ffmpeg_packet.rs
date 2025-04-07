use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FFMPEGPacket {
    pub header: crate::std_msgs::msg::Header,
    pub width: i32,
    pub height: i32,
    pub encoding: ::std::string::String,
    pub pts: u64,
    pub flags: u8,
    pub is_bigendian: bool,
    pub data: Vec<u8>,
}

impl Default for FFMPEGPacket {
    fn default() -> Self {
        FFMPEGPacket {
            header: crate::std_msgs::msg::Header::default(),
            width: 0,
            height: 0,
            encoding: ::std::string::String::new(),
            pts: 0,
            flags: 0,
            is_bigendian: false,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for FFMPEGPacket {}
