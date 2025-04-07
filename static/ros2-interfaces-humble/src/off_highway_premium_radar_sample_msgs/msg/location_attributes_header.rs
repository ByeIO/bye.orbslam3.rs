use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationAttributesHeader {
    pub lgp_version: u32,
    pub block_counter: u8,
    pub start_measurement: crate::off_highway_premium_radar_sample_msgs::msg::Time,
    pub operation_mode: crate::off_highway_premium_radar_sample_msgs::msg::OperationMode,
    pub data_measured: bool,
}

impl Default for LocationAttributesHeader {
    fn default() -> Self {
        LocationAttributesHeader {
            lgp_version: 0,
            block_counter: 0,
            start_measurement: crate::off_highway_premium_radar_sample_msgs::msg::Time::default(),
            operation_mode: crate::off_highway_premium_radar_sample_msgs::msg::OperationMode::default(),
            data_measured: false,
        }
    }
}

impl ros2_client::Message for LocationAttributesHeader {}
