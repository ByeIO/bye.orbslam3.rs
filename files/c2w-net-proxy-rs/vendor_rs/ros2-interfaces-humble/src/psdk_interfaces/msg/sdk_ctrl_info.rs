use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SDKCtrlInfo {
    pub header: crate::std_msgs::msg::Header,
    pub control_mode: u8,
    pub device_status: u8,
    pub flight_status: u8,
    pub vrc_status: u8,
    pub reserved: u8,
}

impl Default for SDKCtrlInfo {
    fn default() -> Self {
        SDKCtrlInfo {
            header: crate::std_msgs::msg::Header::default(),
            control_mode: 0,
            device_status: 0,
            flight_status: 0,
            vrc_status: 0,
            reserved: 0,
        }
    }
}

impl ros2_client::Message for SDKCtrlInfo {}
