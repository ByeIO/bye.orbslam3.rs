use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrStatusSerialNumber {
    pub header: crate::std_msgs::msg::Header,
    pub can_sequence_number: u16,
    pub can_serial_number: u64,
}

impl Default for MrrStatusSerialNumber {
    fn default() -> Self {
        MrrStatusSerialNumber {
            header: crate::std_msgs::msg::Header::default(),
            can_sequence_number: 0,
            can_serial_number: 0,
        }
    }
}

impl ros2_client::Message for MrrStatusSerialNumber {}
