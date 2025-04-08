use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub header: crate::std_msgs::msg::Header,
    pub clock_status: ::std::string::String,
    pub offset: f64,
    pub offset_std: f64,
    pub utc_offset: f64,
    pub utc_year: u32,
    pub utc_month: u8,
    pub utc_day: u8,
    pub utc_hour: u8,
    pub utc_minute: u8,
    pub utc_millisecond: u32,
    pub utc_status: ::std::string::String,
}

impl Default for Time {
    fn default() -> Self {
        Time {
            header: crate::std_msgs::msg::Header::default(),
            clock_status: ::std::string::String::new(),
            offset: 0.0,
            offset_std: 0.0,
            utc_offset: 0.0,
            utc_year: 0,
            utc_month: 0,
            utc_day: 0,
            utc_hour: 0,
            utc_minute: 0,
            utc_millisecond: 0,
            utc_status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Time {}
