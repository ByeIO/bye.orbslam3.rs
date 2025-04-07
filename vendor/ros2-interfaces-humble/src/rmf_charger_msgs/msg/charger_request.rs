use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargerRequest {
    pub charger_name: ::std::string::String,
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub start_timeout: crate::builtin_interfaces::msg::Duration,
    pub request_id: ::std::string::String,
}

impl Default for ChargerRequest {
    fn default() -> Self {
        ChargerRequest {
            charger_name: ::std::string::String::new(),
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            start_timeout: crate::builtin_interfaces::msg::Duration::default(),
            request_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ChargerRequest {}
