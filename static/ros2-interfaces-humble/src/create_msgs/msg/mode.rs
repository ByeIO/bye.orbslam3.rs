use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mode {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u8,
}

impl Mode {
    pub const MODE_OFF: u8 = 0;
    pub const MODE_PASSIVE: u8 = 1;
    pub const MODE_SAFE: u8 = 2;
    pub const MODE_FULL: u8 = 3;
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
        }
    }
}

impl ros2_client::Message for Mode {}
