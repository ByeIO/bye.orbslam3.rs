use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationDataHeader {
    pub header: crate::std_msgs::msg::Header,
    pub start_measurement: crate::off_highway_premium_radar_sample_msgs::msg::Time,
    pub lgp_version: u32,
    pub block_counter: u8,
    pub operation_mode: crate::off_highway_premium_radar_sample_msgs::msg::OperationMode,
    pub data_measured: bool,
    pub num_locations: u16,
}

impl Default for LocationDataHeader {
    fn default() -> Self {
        LocationDataHeader {
            header: crate::std_msgs::msg::Header::default(),
            start_measurement: crate::off_highway_premium_radar_sample_msgs::msg::Time::default(),
            lgp_version: 0,
            block_counter: 0,
            operation_mode: crate::off_highway_premium_radar_sample_msgs::msg::OperationMode::default(),
            data_measured: false,
            num_locations: 0,
        }
    }
}

impl ros2_client::Message for LocationDataHeader {}
