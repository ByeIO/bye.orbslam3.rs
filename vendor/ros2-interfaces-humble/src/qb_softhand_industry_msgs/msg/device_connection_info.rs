use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceConnectionInfo {
    pub is_active: bool,
    pub ip: ::std::string::String,
}

impl Default for DeviceConnectionInfo {
    fn default() -> Self {
        DeviceConnectionInfo {
            is_active: false,
            ip: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeviceConnectionInfo {}
