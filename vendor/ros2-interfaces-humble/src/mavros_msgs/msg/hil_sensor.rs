use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilSensor {
    pub header: crate::std_msgs::msg::Header,
    pub acc: crate::geometry_msgs::msg::Vector3,
    pub gyro: crate::geometry_msgs::msg::Vector3,
    pub mag: crate::geometry_msgs::msg::Vector3,
    pub abs_pressure: f32,
    pub diff_pressure: f32,
    pub pressure_alt: f32,
    pub temperature: f32,
    pub fields_updated: u32,
}

impl Default for HilSensor {
    fn default() -> Self {
        HilSensor {
            header: crate::std_msgs::msg::Header::default(),
            acc: crate::geometry_msgs::msg::Vector3::default(),
            gyro: crate::geometry_msgs::msg::Vector3::default(),
            mag: crate::geometry_msgs::msg::Vector3::default(),
            abs_pressure: 0.0,
            diff_pressure: 0.0,
            pressure_alt: 0.0,
            temperature: 0.0,
            fields_updated: 0,
        }
    }
}

impl ros2_client::Message for HilSensor {}
