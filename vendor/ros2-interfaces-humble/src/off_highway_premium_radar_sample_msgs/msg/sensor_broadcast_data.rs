use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorBroadcastData {
    pub customer_version: u32,
    pub sensor_ethernet_configuration_information: crate::off_highway_premium_radar_sample_msgs::msg::SensorEthernetConfigurationInformation,
    pub diagnostics_ethernet_configuration_information: crate::off_highway_premium_radar_sample_msgs::msg::DiagnosticsEthernetConfigurationInformation,
    pub sensor_mac_address: u64,
    pub doip_information: crate::off_highway_premium_radar_sample_msgs::msg::DoIpInformation,
}

impl Default for SensorBroadcastData {
    fn default() -> Self {
        SensorBroadcastData {
            customer_version: 0,
            sensor_ethernet_configuration_information: crate::off_highway_premium_radar_sample_msgs::msg::SensorEthernetConfigurationInformation::default(),
            diagnostics_ethernet_configuration_information: crate::off_highway_premium_radar_sample_msgs::msg::DiagnosticsEthernetConfigurationInformation::default(),
            sensor_mac_address: 0,
            doip_information: crate::off_highway_premium_radar_sample_msgs::msg::DoIpInformation::default(),
        }
    }
}

impl ros2_client::Message for SensorBroadcastData {}
