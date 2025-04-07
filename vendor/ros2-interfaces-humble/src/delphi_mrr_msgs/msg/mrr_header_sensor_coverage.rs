use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrHeaderSensorCoverage {
    pub header: crate::std_msgs::msg::Header,
    pub can_sensor_fov_hor: u8,
    pub can_doppler_coverage: i8,
    pub can_range_coverage: u8,
}

impl Default for MrrHeaderSensorCoverage {
    fn default() -> Self {
        MrrHeaderSensorCoverage {
            header: crate::std_msgs::msg::Header::default(),
            can_sensor_fov_hor: 0,
            can_doppler_coverage: 0,
            can_range_coverage: 0,
        }
    }
}

impl ros2_client::Message for MrrHeaderSensorCoverage {}
