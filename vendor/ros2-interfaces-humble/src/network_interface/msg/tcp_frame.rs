use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TCPFrame {
    pub header: crate::std_msgs::msg::Header,
    pub address: ::std::string::String,
    pub port: u16,
    pub size: u16,
    pub data: Vec<u8>,
}

impl Default for TCPFrame {
    fn default() -> Self {
        TCPFrame {
            header: crate::std_msgs::msg::Header::default(),
            address: ::std::string::String::new(),
            port: 0,
            size: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for TCPFrame {}
