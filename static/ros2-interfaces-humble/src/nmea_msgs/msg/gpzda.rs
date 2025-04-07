use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpzda {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub utc_seconds: u32,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub hour_offset_gmt: i8,
    pub minute_offset_gmt: u8,
}

impl Default for Gpzda {
    fn default() -> Self {
        Gpzda {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            utc_seconds: 0,
            day: 0,
            month: 0,
            year: 0,
            hour_offset_gmt: 0,
            minute_offset_gmt: 0,
        }
    }
}

impl ros2_client::Message for Gpzda {}
