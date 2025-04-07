use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusEvent {
    pub header: crate::std_msgs::msg::Header,
    pub severity: u8,
    pub px4_id: u32,
    #[serde_as(as = "[_; 40]")]
    pub arguments: [u8; 40],
    pub sequence: u16,
}

impl StatusEvent {
    pub const EMERGENCY: u8 = 0;
    pub const ALERT: u8 = 1;
    pub const CRITICAL: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const WARNING: u8 = 4;
    pub const NOTICE: u8 = 5;
    pub const INFO: u8 = 6;
    pub const DEBUG: u8 = 7;
}

impl Default for StatusEvent {
    fn default() -> Self {
        StatusEvent {
            header: crate::std_msgs::msg::Header::default(),
            severity: 0,
            px4_id: 0,
            arguments: [0; 40],
            sequence: 0,
        }
    }
}

impl ros2_client::Message for StatusEvent {}
