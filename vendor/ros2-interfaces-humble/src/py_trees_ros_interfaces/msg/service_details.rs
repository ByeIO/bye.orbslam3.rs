use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDetails {
    pub service_name: ::std::string::String,
    pub service_type: ::std::string::String,
}

impl Default for ServiceDetails {
    fn default() -> Self {
        ServiceDetails {
            service_name: ::std::string::String::new(),
            service_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceDetails {}
