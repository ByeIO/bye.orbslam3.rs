use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EgoVehicleInput {
    pub header: crate::std_msgs::msg::Header,
    pub vehicle_data: crate::off_highway_premium_radar_sample_msgs::msg::EgoVehicleData,
}

impl Default for EgoVehicleInput {
    fn default() -> Self {
        EgoVehicleInput {
            header: crate::std_msgs::msg::Header::default(),
            vehicle_data: crate::off_highway_premium_radar_sample_msgs::msg::EgoVehicleData::default(),
        }
    }
}

impl ros2_client::Message for EgoVehicleInput {}
