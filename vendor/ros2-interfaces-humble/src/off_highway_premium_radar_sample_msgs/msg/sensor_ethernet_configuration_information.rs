use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorEthernetConfigurationInformation {
    pub sensor_ip_address: ::std::string::String,
    pub destination_ip_address: ::std::string::String,
    pub netmask: ::std::string::String,
    pub vlan: u16,
    pub source_port: u16,
    pub destination_port: u16,
}

impl Default for SensorEthernetConfigurationInformation {
    fn default() -> Self {
        SensorEthernetConfigurationInformation {
            sensor_ip_address: ::std::string::String::new(),
            destination_ip_address: ::std::string::String::new(),
            netmask: ::std::string::String::new(),
            vlan: 0,
            source_port: 0,
            destination_port: 0,
        }
    }
}

impl ros2_client::Message for SensorEthernetConfigurationInformation {}
