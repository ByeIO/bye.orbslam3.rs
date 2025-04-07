use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectedClient {
    pub ip_address: ::std::string::String,
    pub connection_time: crate::builtin_interfaces::msg::Time,
}

impl Default for ConnectedClient {
    fn default() -> Self {
        ConnectedClient {
            ip_address: ::std::string::String::new(),
            connection_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for ConnectedClient {}
