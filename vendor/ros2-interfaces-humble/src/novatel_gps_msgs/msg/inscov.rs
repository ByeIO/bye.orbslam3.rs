use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inscov {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub week: u32,
    pub seconds: f64,
    pub position_covariance: [f64; 9],
    pub attitude_covariance: [f64; 9],
    pub velocity_covariance: [f64; 9],
}

impl Default for Inscov {
    fn default() -> Self {
        Inscov {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            week: 0,
            seconds: 0.0,
            position_covariance: [0.0; 9],
            attitude_covariance: [0.0; 9],
            velocity_covariance: [0.0; 9],
        }
    }
}

impl ros2_client::Message for Inscov {}
