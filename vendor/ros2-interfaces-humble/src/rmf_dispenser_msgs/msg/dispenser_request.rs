use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispenserRequest {
    pub time: crate::builtin_interfaces::msg::Time,
    pub request_guid: ::std::string::String,
    pub target_guid: ::std::string::String,
    pub transporter_type: ::std::string::String,
    pub items: Vec<crate::rmf_dispenser_msgs::msg::DispenserRequestItem>,
}

impl Default for DispenserRequest {
    fn default() -> Self {
        DispenserRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
            request_guid: ::std::string::String::new(),
            target_guid: ::std::string::String::new(),
            transporter_type: ::std::string::String::new(),
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for DispenserRequest {}
