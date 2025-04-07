use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub id: i32,
    pub parent: i32, // default: -1
    pub name: ::std::string::String,
    pub transitions: Vec<crate::yasmin_msgs::msg::Transition>,
    pub outcomes: Vec<::std::string::String>,
    pub is_fsm: bool,
    pub current_state: i32, // default: -1
}

impl Default for State {
    fn default() -> Self {
        State {
            id: 0,
            parent: -1,
            name: ::std::string::String::new(),
            transitions: Vec::new(),
            outcomes: Vec::new(),
            is_fsm: false,
            current_state: -1,
        }
    }
}

impl ros2_client::Message for State {}
