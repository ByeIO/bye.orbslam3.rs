use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccTransition {
    pub index: i32,
    pub transition_name: ::std::string::String,
    pub transition_type: ::std::string::String,
    pub destiny_state_name: ::std::string::String,
    pub source_state_name: ::std::string::String,
    pub history_node: bool,
    pub event: crate::smacc2_msgs::msg::SmaccEvent,
}

impl Default for SmaccTransition {
    fn default() -> Self {
        SmaccTransition {
            index: 0,
            transition_name: ::std::string::String::new(),
            transition_type: ::std::string::String::new(),
            destiny_state_name: ::std::string::String::new(),
            source_state_name: ::std::string::String::new(),
            history_node: false,
            event: crate::smacc2_msgs::msg::SmaccEvent::default(),
        }
    }
}

impl ros2_client::Message for SmaccTransition {}
