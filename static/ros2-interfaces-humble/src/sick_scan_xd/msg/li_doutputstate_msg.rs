use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateMsg {
    pub header: crate::std_msgs::msg::Header,
    pub version_number: u16,
    pub system_counter: u32,
    pub output_state: Vec<u8>,
    pub output_count: Vec<u32>,
    pub time_state: u16,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub microsecond: u32,
}

impl Default for LIDoutputstateMsg {
    fn default() -> Self {
        LIDoutputstateMsg {
            header: crate::std_msgs::msg::Header::default(),
            version_number: 0,
            system_counter: 0,
            output_state: Vec::new(),
            output_count: Vec::new(),
            time_state: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            microsecond: 0,
        }
    }
}

impl ros2_client::Message for LIDoutputstateMsg {}
