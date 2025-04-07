use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccContainerInitialStatusCmd {
    pub path: ::std::string::String,
    pub initial_states: Vec<::std::string::String>,
    pub local_data: ::std::string::String,
}

impl Default for SmaccContainerInitialStatusCmd {
    fn default() -> Self {
        SmaccContainerInitialStatusCmd {
            path: ::std::string::String::new(),
            initial_states: Vec::new(),
            local_data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SmaccContainerInitialStatusCmd {}
