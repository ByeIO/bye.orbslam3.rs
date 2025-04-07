use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IbeoDataHeader {
    pub previous_message_size: u32,
    pub message_size: u32,
    pub device_id: u8,
    pub data_type_id: u16,
    pub stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for IbeoDataHeader {
    fn default() -> Self {
        IbeoDataHeader {
            previous_message_size: 0,
            message_size: 0,
            device_id: 0,
            data_type_id: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for IbeoDataHeader {}
