use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockInfraRed {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u8>,
}

impl DockInfraRed {
    pub const NEAR_LEFT: u8 = 1;
    pub const NEAR_CENTER: u8 = 2;
    pub const NEAR_RIGHT: u8 = 4;
    pub const FAR_LEFT: u8 = 16;
    pub const FAR_CENTER: u8 = 8;
    pub const FAR_RIGHT: u8 = 32;
}

impl Default for DockInfraRed {
    fn default() -> Self {
        DockInfraRed {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for DockInfraRed {}
