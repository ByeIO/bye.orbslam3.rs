use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub scanner_type: u8,
    pub sensor_temperature: f32,
    pub frequency: f32,
}

impl Default for DeviceStatus {
    fn default() -> Self {
        DeviceStatus {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            scanner_type: 0,
            sensor_temperature: 0.0,
            frequency: 0.0,
        }
    }
}

impl ros2_client::Message for DeviceStatus {}
