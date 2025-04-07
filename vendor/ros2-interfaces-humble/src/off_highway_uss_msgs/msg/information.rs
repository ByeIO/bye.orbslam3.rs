use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Information {
    pub header: crate::std_msgs::msg::Header,
    pub number_sensors: u8,
    pub sending_pattern: u8,
    pub operating_mode: u8,
    pub outside_temperature: f64,
    pub sensor_blindness: u16,
    pub sensitivity: u8,
    pub sensor_faulted: u16,
    pub failure_status: bool,
}

impl Default for Information {
    fn default() -> Self {
        Information {
            header: crate::std_msgs::msg::Header::default(),
            number_sensors: 0,
            sending_pattern: 0,
            operating_mode: 0,
            outside_temperature: 0.0,
            sensor_blindness: 0,
            sensitivity: 0,
            sensor_faulted: 0,
            failure_status: false,
        }
    }
}

impl ros2_client::Message for Information {}
