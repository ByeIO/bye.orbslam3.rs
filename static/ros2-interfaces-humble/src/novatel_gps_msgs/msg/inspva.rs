use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inspva {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub week: u32,
    pub seconds: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub north_velocity: f64,
    pub east_velocity: f64,
    pub up_velocity: f64,
    pub roll: f64,
    pub pitch: f64,
    pub azimuth: f64,
    pub status: ::std::string::String,
}

impl Default for Inspva {
    fn default() -> Self {
        Inspva {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            week: 0,
            seconds: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            height: 0.0,
            north_velocity: 0.0,
            east_velocity: 0.0,
            up_velocity: 0.0,
            roll: 0.0,
            pitch: 0.0,
            azimuth: 0.0,
            status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Inspva {}
