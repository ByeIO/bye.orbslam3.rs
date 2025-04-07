use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activation {
    pub operation_type: u8,
    pub activator: ::std::string::String,
    pub activation: ::std::string::String,
}

impl Activation {
    pub const ADD: u8 = 0;
    pub const REMOVE: u8 = 1;
}

impl Default for Activation {
    fn default() -> Self {
        Activation {
            operation_type: 0,
            activator: ::std::string::String::new(),
            activation: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Activation {}
