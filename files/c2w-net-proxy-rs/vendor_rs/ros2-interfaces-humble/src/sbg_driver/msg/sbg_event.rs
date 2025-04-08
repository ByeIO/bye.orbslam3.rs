use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEvent {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub overflow: bool,
    pub offset_0_valid: bool,
    pub offset_1_valid: bool,
    pub offset_2_valid: bool,
    pub offset_3_valid: bool,
    pub time_offset_0: u16,
    pub time_offset_1: u16,
    pub time_offset_2: u16,
    pub time_offset_3: u16,
}

impl Default for SbgEvent {
    fn default() -> Self {
        SbgEvent {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            overflow: false,
            offset_0_valid: false,
            offset_1_valid: false,
            offset_2_valid: false,
            offset_3_valid: false,
            time_offset_0: 0,
            time_offset_1: 0,
            time_offset_2: 0,
            time_offset_3: 0,
        }
    }
}

impl ros2_client::Message for SbgEvent {}
