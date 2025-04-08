use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngestorRequest {
    pub time: crate::builtin_interfaces::msg::Time,
    pub request_guid: ::std::string::String,
    pub target_guid: ::std::string::String,
    pub transporter_type: ::std::string::String,
    pub items: Vec<crate::rmf_ingestor_msgs::msg::IngestorRequestItem>,
}

impl Default for IngestorRequest {
    fn default() -> Self {
        IngestorRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
            request_guid: ::std::string::String::new(),
            target_guid: ::std::string::String::new(),
            transporter_type: ::std::string::String::new(),
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for IngestorRequest {}
