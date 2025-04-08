use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrHeaderSensorPosition {
    pub header: crate::std_msgs::msg::Header,
    pub can_sensor_polarity: bool,
    pub can_sensor_lat_offset: f32,
    pub can_sensor_long_offset: f32,
    pub can_sensor_hangle_offset: f32,
}

impl Default for MrrHeaderSensorPosition {
    fn default() -> Self {
        MrrHeaderSensorPosition {
            header: crate::std_msgs::msg::Header::default(),
            can_sensor_polarity: false,
            can_sensor_lat_offset: 0.0,
            can_sensor_long_offset: 0.0,
            can_sensor_hangle_offset: 0.0,
        }
    }
}

impl ros2_client::Message for MrrHeaderSensorPosition {}
