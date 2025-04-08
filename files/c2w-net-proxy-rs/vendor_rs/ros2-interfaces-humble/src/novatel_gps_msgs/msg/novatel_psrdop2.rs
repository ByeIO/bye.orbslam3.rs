use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelPsrdop2 {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub gdop: f32,
    pub pdop: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub systems: Vec<crate::novatel_gps_msgs::msg::NovatelPsrdop2System>,
}

impl Default for NovatelPsrdop2 {
    fn default() -> Self {
        NovatelPsrdop2 {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            gdop: 0.0,
            pdop: 0.0,
            hdop: 0.0,
            vdop: 0.0,
            systems: Vec::new(),
        }
    }
}

impl ros2_client::Message for NovatelPsrdop2 {}
