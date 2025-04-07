use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerState {
    pub name: ::std::string::String,
    pub state: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub claimed_interfaces: Vec<::std::string::String>,
    pub required_command_interfaces: Vec<::std::string::String>,
    pub required_state_interfaces: Vec<::std::string::String>,
    pub is_chainable: bool,
    pub is_chained: bool,
    pub reference_interfaces: Vec<::std::string::String>,
    pub chain_connections: Vec<crate::controller_manager_msgs::msg::ChainConnection>,
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            name: ::std::string::String::new(),
            state: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            claimed_interfaces: Vec::new(),
            required_command_interfaces: Vec::new(),
            required_state_interfaces: Vec::new(),
            is_chainable: false,
            is_chained: false,
            reference_interfaces: Vec::new(),
            chain_connections: Vec::new(),
        }
    }
}

impl ros2_client::Message for ControllerState {}
