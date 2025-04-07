use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub is_rtr: bool,
    pub is_extended: bool,
    pub is_error: bool,
    pub dlc: u8,
    pub data: [u8; 8],
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            is_rtr: false,
            is_extended: false,
            is_error: false,
            dlc: 0,
            data: [0; 8],
        }
    }
}

impl ros2_client::Message for Frame {}
