use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: ::std::string::String,
    pub resolved_name: ::std::string::String,
    pub description: ::std::string::String,
    pub group: ::std::string::String,
    pub message_type: ::std::string::String,
    pub server: bool,
    pub topic_service: bool,
}

impl Default for ServiceInfo {
    fn default() -> Self {
        ServiceInfo {
            name: ::std::string::String::new(),
            resolved_name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            group: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            server: false,
            topic_service: false,
        }
    }
}

impl ros2_client::Message for ServiceInfo {}
