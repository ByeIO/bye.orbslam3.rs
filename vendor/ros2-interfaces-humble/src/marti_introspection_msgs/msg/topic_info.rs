use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicInfo {
    pub name: ::std::string::String,
    pub resolved_name: ::std::string::String,
    pub description: ::std::string::String,
    pub group: ::std::string::String,
    pub message_type: ::std::string::String,
    pub advertised: bool,
}

impl Default for TopicInfo {
    fn default() -> Self {
        TopicInfo {
            name: ::std::string::String::new(),
            resolved_name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            group: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            advertised: false,
        }
    }
}

impl ros2_client::Message for TopicInfo {}
