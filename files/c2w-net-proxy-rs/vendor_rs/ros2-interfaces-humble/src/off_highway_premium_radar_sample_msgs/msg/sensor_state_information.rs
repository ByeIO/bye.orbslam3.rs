use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorStateInformation {
    pub header: crate::std_msgs::msg::Header,
    pub lgp_version: u32,
    pub sensor_state: u8,
    pub customer_version: u32,
    pub internal_version: [u8; 5],
}

impl Default for SensorStateInformation {
    fn default() -> Self {
        SensorStateInformation {
            header: crate::std_msgs::msg::Header::default(),
            lgp_version: 0,
            sensor_state: 0,
            customer_version: 0,
            internal_version: [0; 5],
        }
    }
}

impl ros2_client::Message for SensorStateInformation {}
