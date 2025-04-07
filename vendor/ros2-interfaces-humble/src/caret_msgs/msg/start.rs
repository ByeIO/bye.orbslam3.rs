use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Start {
    pub recording_frequency: u32, // default: 100
    pub ignore_nodes: ::std::string::String,
    pub ignore_topics: ::std::string::String,
    pub select_nodes: ::std::string::String,
    pub select_topics: ::std::string::String,
}

impl Default for Start {
    fn default() -> Self {
        Start {
            recording_frequency: 100,
            ignore_nodes: ::std::string::String::new(),
            ignore_topics: ::std::string::String::new(),
            select_nodes: ::std::string::String::new(),
            select_topics: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Start {}
