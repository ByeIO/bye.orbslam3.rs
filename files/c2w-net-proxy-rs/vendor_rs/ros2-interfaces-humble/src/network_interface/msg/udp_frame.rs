use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UDPFrame {
    pub header: crate::std_msgs::msg::Header,
    pub address: ::std::string::String,
    pub port: u16,
    pub data: Vec<u8>,
}

impl Default for UDPFrame {
    fn default() -> Self {
        UDPFrame {
            header: crate::std_msgs::msg::Header::default(),
            address: ::std::string::String::new(),
            port: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UDPFrame {}
