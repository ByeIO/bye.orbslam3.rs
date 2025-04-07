use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrStatus3 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_alignment_state: u8,
    pub can_tx_interface_ver_minor: u8,
    pub can_tx_sw_version_arm: u32,
    pub can_tx_hw_version: u8,
    pub can_tx_interface_version: u8,
    pub can_tx_serial_num: u32,
}

impl SrrStatus3 {
    pub const CAN_TX_ALIGNMENT_STATE_OFF: u8 = 0;
    pub const CAN_TX_ALIGNMENT_STATE_INIT: u8 = 1;
    pub const CAN_TX_ALIGNMENT_STATE_AUTOMATIC_ALIGNMENT: u8 = 2;
    pub const CAN_TX_ALIGNMENT_STATE_FACTORY_ALIGNMENT: u8 = 3;
    pub const CAN_TX_ALIGNMENT_STATE_SERVICE_ALIGNMENT: u8 = 4;
}

impl Default for SrrStatus3 {
    fn default() -> Self {
        SrrStatus3 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_alignment_state: 0,
            can_tx_interface_ver_minor: 0,
            can_tx_sw_version_arm: 0,
            can_tx_hw_version: 0,
            can_tx_interface_version: 0,
            can_tx_serial_num: 0,
        }
    }
}

impl ros2_client::Message for SrrStatus3 {}
