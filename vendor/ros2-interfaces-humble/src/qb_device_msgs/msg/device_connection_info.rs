use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceConnectionInfo {
    pub id: i32,
    pub is_active: bool,
    pub port: ::std::string::String,
}

impl Default for DeviceConnectionInfo {
    fn default() -> Self {
        DeviceConnectionInfo {
            id: 0,
            is_active: false,
            port: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeviceConnectionInfo {}
