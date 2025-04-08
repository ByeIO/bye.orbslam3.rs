use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccState {
    pub index: i32,
    pub name: ::std::string::String,
    pub children_states: Vec<::std::string::String>,
    pub level: i8,
    pub transitions: Vec<crate::smacc2_msgs::msg::SmaccTransition>,
    pub orthogonals: Vec<crate::smacc2_msgs::msg::SmaccOrthogonal>,
    pub state_reactors: Vec<crate::smacc2_msgs::msg::SmaccStateReactor>,
    pub event_generators: Vec<crate::smacc2_msgs::msg::SmaccEventGenerator>,
}

impl Default for SmaccState {
    fn default() -> Self {
        SmaccState {
            index: 0,
            name: ::std::string::String::new(),
            children_states: Vec::new(),
            level: 0,
            transitions: Vec::new(),
            orthogonals: Vec::new(),
            state_reactors: Vec::new(),
            event_generators: Vec::new(),
        }
    }
}

impl ros2_client::Message for SmaccState {}
