use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelMessageHeader {
    pub message_name: ::std::string::String,
    pub port: ::std::string::String,
    pub sequence_num: u32,
    pub percent_idle_time: f32,
    pub gps_time_status: ::std::string::String,
    pub gps_week_num: u32,
    pub gps_seconds: f64,
    pub receiver_status: crate::novatel_gps_msgs::msg::NovatelReceiverStatus,
    pub receiver_software_version: u32,
}

impl Default for NovatelMessageHeader {
    fn default() -> Self {
        NovatelMessageHeader {
            message_name: ::std::string::String::new(),
            port: ::std::string::String::new(),
            sequence_num: 0,
            percent_idle_time: 0.0,
            gps_time_status: ::std::string::String::new(),
            gps_week_num: 0,
            gps_seconds: 0.0,
            receiver_status: crate::novatel_gps_msgs::msg::NovatelReceiverStatus::default(),
            receiver_software_version: 0,
        }
    }
}

impl ros2_client::Message for NovatelMessageHeader {}
