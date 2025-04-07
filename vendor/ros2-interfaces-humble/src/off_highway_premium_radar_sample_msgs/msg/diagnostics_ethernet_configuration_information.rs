use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsEthernetConfigurationInformation {
    pub ip_address: ::std::string::String,
    pub netmask: ::std::string::String,
    pub vlan: u16,
    pub port: u16,
}

impl Default for DiagnosticsEthernetConfigurationInformation {
    fn default() -> Self {
        DiagnosticsEthernetConfigurationInformation {
            ip_address: ::std::string::String::new(),
            netmask: ::std::string::String::new(),
            vlan: 0,
            port: 0,
        }
    }
}

impl ros2_client::Message for DiagnosticsEthernetConfigurationInformation {}
