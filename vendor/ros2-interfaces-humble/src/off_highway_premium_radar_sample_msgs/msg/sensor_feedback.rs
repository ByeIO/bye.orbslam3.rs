use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub lgp_version: u32,
    pub vehicle_time: crate::off_highway_premium_radar_sample_msgs::msg::Time,
    pub measurement_cycle_sync_data: crate::off_highway_premium_radar_sample_msgs::msg::MeasurementCycleSyncData,
    pub time_sync_status: u8,
    pub ego_vehicle_data: crate::off_highway_premium_radar_sample_msgs::msg::EgoVehicleData,
}

impl Default for SensorFeedback {
    fn default() -> Self {
        SensorFeedback {
            header: crate::std_msgs::msg::Header::default(),
            lgp_version: 0,
            vehicle_time: crate::off_highway_premium_radar_sample_msgs::msg::Time::default(),
            measurement_cycle_sync_data: crate::off_highway_premium_radar_sample_msgs::msg::MeasurementCycleSyncData::default(),
            time_sync_status: 0,
            ego_vehicle_data: crate::off_highway_premium_radar_sample_msgs::msg::EgoVehicleData::default(),
        }
    }
}

impl ros2_client::Message for SensorFeedback {}
