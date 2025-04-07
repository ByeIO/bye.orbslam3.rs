use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inspvax {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub ins_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub undulation: f32,
    pub north_velocity: f64,
    pub east_velocity: f64,
    pub up_velocity: f64,
    pub roll: f64,
    pub pitch: f64,
    pub azimuth: f64,
    pub latitude_std: f32,
    pub longitude_std: f32,
    pub altitude_std: f32,
    pub north_velocity_std: f32,
    pub east_velocity_std: f32,
    pub up_velocity_std: f32,
    pub roll_std: f32,
    pub pitch_std: f32,
    pub azimuth_std: f32,
    pub extended_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus,
    pub seconds_since_update: u16,
}

impl Default for Inspvax {
    fn default() -> Self {
        Inspvax {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            ins_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            undulation: 0.0,
            north_velocity: 0.0,
            east_velocity: 0.0,
            up_velocity: 0.0,
            roll: 0.0,
            pitch: 0.0,
            azimuth: 0.0,
            latitude_std: 0.0,
            longitude_std: 0.0,
            altitude_std: 0.0,
            north_velocity_std: 0.0,
            east_velocity_std: 0.0,
            up_velocity_std: 0.0,
            roll_std: 0.0,
            pitch_std: 0.0,
            azimuth_std: 0.0,
            extended_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus::default(),
            seconds_since_update: 0,
        }
    }
}

impl ros2_client::Message for Inspvax {}
