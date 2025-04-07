use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub hostname: ::std::string::String,
    pub lidar_mode: ::std::string::String,
    pub timestamp_mode: ::std::string::String,
    pub beam_azimuth_angles: Vec<f64>,
    pub beam_altitude_angles: Vec<f64>,
    pub imu_to_sensor_transform: Vec<f64>,
    pub lidar_to_sensor_transform: Vec<f64>,
    pub serial_no: ::std::string::String,
    pub firmware_rev: ::std::string::String,
    pub imu_port: i8,
    pub lidar_port: i8,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            hostname: ::std::string::String::new(),
            lidar_mode: ::std::string::String::new(),
            timestamp_mode: ::std::string::String::new(),
            beam_azimuth_angles: Vec::new(),
            beam_altitude_angles: Vec::new(),
            imu_to_sensor_transform: Vec::new(),
            lidar_to_sensor_transform: Vec::new(),
            serial_no: ::std::string::String::new(),
            firmware_rev: ::std::string::String::new(),
            imu_port: 0,
            lidar_port: 0,
        }
    }
}

impl ros2_client::Message for Metadata {}
