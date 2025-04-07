use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgAirData {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: crate::sbg_driver::msg::SbgAirDataStatus,
    pub pressure_abs: f64,
    pub altitude: f64,
    pub pressure_diff: f64,
    pub true_air_speed: f64,
    pub air_temperature: f64,
}

impl Default for SbgAirData {
    fn default() -> Self {
        SbgAirData {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: crate::sbg_driver::msg::SbgAirDataStatus::default(),
            pressure_abs: 0.0,
            altitude: 0.0,
            pressure_diff: 0.0,
            true_air_speed: 0.0,
            air_temperature: 0.0,
        }
    }
}

impl ros2_client::Message for SbgAirData {}
