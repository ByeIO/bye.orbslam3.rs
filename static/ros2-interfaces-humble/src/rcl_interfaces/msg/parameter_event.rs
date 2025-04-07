use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterEvent {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub node: ::std::string::String,
    pub new_parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
    pub changed_parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
    pub deleted_parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for ParameterEvent {
    fn default() -> Self {
        ParameterEvent {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            node: ::std::string::String::new(),
            new_parameters: Vec::new(),
            changed_parameters: Vec::new(),
            deleted_parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParameterEvent {}
