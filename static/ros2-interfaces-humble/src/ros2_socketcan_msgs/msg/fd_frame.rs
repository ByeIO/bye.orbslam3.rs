use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FdFrame {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub is_extended: bool,
    pub is_error: bool,
    pub len: u8,
    pub data: Vec<u8>,
}

impl Default for FdFrame {
    fn default() -> Self {
        FdFrame {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            is_extended: false,
            is_error: false,
            len: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for FdFrame {}
