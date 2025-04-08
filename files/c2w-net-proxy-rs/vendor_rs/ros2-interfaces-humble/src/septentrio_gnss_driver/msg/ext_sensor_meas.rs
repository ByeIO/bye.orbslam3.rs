use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtSensorMeas {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub n: u8,
    pub sb_length: u8,
    pub source: Vec<u8>,
    pub sensor_model: Vec<u8>,
    #[serde(rename = "type")]    pub type_: Vec<u8>,
    pub obs_info: Vec<u8>,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub acceleration_z: f64,
    pub angular_rate_x: f64,
    pub angular_rate_y: f64,
    pub angular_rate_z: f64,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub std_dev_x: f32,
    pub std_dev_y: f32,
    pub std_dev_z: f32,
    pub sensor_temperature: f32,
    pub zero_velocity_flag: f64,
}

impl Default for ExtSensorMeas {
    fn default() -> Self {
        ExtSensorMeas {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            n: 0,
            sb_length: 0,
            source: Vec::new(),
            sensor_model: Vec::new(),
            type_: Vec::new(),
            obs_info: Vec::new(),
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            acceleration_z: 0.0,
            angular_rate_x: 0.0,
            angular_rate_y: 0.0,
            angular_rate_z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            std_dev_x: 0.0,
            std_dev_y: 0.0,
            std_dev_z: 0.0,
            sensor_temperature: 0.0,
            zero_velocity_flag: 0.0,
        }
    }
}

impl ros2_client::Message for ExtSensorMeas {}
