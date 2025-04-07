use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerState {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub state: u8,
}

impl ControllerState {
    pub const STOPPED: u8 = 0;
    pub const RUNNING: u8 = 1;
    pub const ERROR: u8 = 2;
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            name: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            state: 0,
        }
    }
}

impl ros2_client::Message for ControllerState {}
