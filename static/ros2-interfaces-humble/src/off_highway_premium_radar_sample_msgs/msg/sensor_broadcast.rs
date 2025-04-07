use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorBroadcast {
    pub header: crate::std_msgs::msg::Header,
    pub lgp_version: u32,
    pub sensor_broadcast_data: crate::off_highway_premium_radar_sample_msgs::msg::SensorBroadcastData,
}

impl Default for SensorBroadcast {
    fn default() -> Self {
        SensorBroadcast {
            header: crate::std_msgs::msg::Header::default(),
            lgp_version: 0,
            sensor_broadcast_data: crate::off_highway_premium_radar_sample_msgs::msg::SensorBroadcastData::default(),
        }
    }
}

impl ros2_client::Message for SensorBroadcast {}
