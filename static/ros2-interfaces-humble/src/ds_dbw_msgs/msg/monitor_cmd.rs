use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd_type: u8,
}

impl MonitorCmd {
    pub const NONE: u8 = 0;
    pub const ACTIVATE_TEST_FAULT: u8 = 1;
    pub const CLEAR_TEST_FAULT: u8 = 2;
}

impl Default for MonitorCmd {
    fn default() -> Self {
        MonitorCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd_type: 0,
        }
    }
}

impl ros2_client::Message for MonitorCmd {}
