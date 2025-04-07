use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionPerformerStatus {
    pub status_stamp: crate::builtin_interfaces::msg::Time,
    pub state: i8,
    pub action: ::std::string::String,
    pub specialized_arguments: Vec<::std::string::String>,
    pub node_name: ::std::string::String,
}

impl ActionPerformerStatus {
    pub const NOT_READY: i8 = 0;
    pub const READY: i8 = 1;
    pub const RUNNING: i8 = 2;
    pub const FAILURE: i8 = 4;
}

impl Default for ActionPerformerStatus {
    fn default() -> Self {
        ActionPerformerStatus {
            status_stamp: crate::builtin_interfaces::msg::Time::default(),
            state: 0,
            action: ::std::string::String::new(),
            specialized_arguments: Vec::new(),
            node_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionPerformerStatus {}
