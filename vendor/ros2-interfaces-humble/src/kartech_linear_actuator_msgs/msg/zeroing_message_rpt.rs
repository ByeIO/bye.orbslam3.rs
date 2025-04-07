use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZeroingMessageRpt {
    pub header: crate::std_msgs::msg::Header,
    pub chip_1_voltage: u16,
    pub chip_2_voltage: u16,
    pub chip_error_1: u8,
    pub chip_error_2: u8,
}

impl Default for ZeroingMessageRpt {
    fn default() -> Self {
        ZeroingMessageRpt {
            header: crate::std_msgs::msg::Header::default(),
            chip_1_voltage: 0,
            chip_2_voltage: 0,
            chip_error_1: 0,
            chip_error_2: 0,
        }
    }
}

impl ros2_client::Message for ZeroingMessageRpt {}
