use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDataProcessor {
    pub data_source_node: ::std::string::String,
    pub user_friendly_source_name: ::std::string::String,
    pub data_purpose: ::std::string::String,
    pub path: ::std::string::String,
}

impl Default for PersonalDataProcessor {
    fn default() -> Self {
        PersonalDataProcessor {
            data_source_node: ::std::string::String::new(),
            user_friendly_source_name: ::std::string::String::new(),
            data_purpose: ::std::string::String::new(),
            path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PersonalDataProcessor {}
