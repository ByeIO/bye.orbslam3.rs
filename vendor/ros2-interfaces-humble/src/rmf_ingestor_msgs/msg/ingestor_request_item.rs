use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngestorRequestItem {
    pub type_guid: ::std::string::String,
    pub quantity: i32,
    pub compartment_name: ::std::string::String,
}

impl Default for IngestorRequestItem {
    fn default() -> Self {
        IngestorRequestItem {
            type_guid: ::std::string::String::new(),
            quantity: 0,
            compartment_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for IngestorRequestItem {}
