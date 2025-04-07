use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelCorrectedImuData {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub gps_week_num: u32,
    pub gps_seconds: f64,
    pub pitch_rate: f64,
    pub roll_rate: f64,
    pub yaw_rate: f64,
    pub lateral_acceleration: f64,
    pub longitudinal_acceleration: f64,
    pub vertical_acceleration: f64,
}

impl Default for NovatelCorrectedImuData {
    fn default() -> Self {
        NovatelCorrectedImuData {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            gps_week_num: 0,
            gps_seconds: 0.0,
            pitch_rate: 0.0,
            roll_rate: 0.0,
            yaw_rate: 0.0,
            lateral_acceleration: 0.0,
            longitudinal_acceleration: 0.0,
            vertical_acceleration: 0.0,
        }
    }
}

impl ros2_client::Message for NovatelCorrectedImuData {}
