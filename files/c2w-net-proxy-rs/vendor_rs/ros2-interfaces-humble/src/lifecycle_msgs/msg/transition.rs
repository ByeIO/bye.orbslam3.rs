use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub id: u8,
    pub label: ::std::string::String,
}

impl Transition {
    pub const TRANSITION_CREATE: u8 = 0;
    pub const TRANSITION_CONFIGURE: u8 = 1;
    pub const TRANSITION_CLEANUP: u8 = 2;
    pub const TRANSITION_ACTIVATE: u8 = 3;
    pub const TRANSITION_DEACTIVATE: u8 = 4;
    pub const TRANSITION_UNCONFIGURED_SHUTDOWN: u8 = 5;
    pub const TRANSITION_INACTIVE_SHUTDOWN: u8 = 6;
    pub const TRANSITION_ACTIVE_SHUTDOWN: u8 = 7;
    pub const TRANSITION_DESTROY: u8 = 8;
    pub const TRANSITION_ON_CONFIGURE_SUCCESS: u8 = 10;
    pub const TRANSITION_ON_CONFIGURE_FAILURE: u8 = 11;
    pub const TRANSITION_ON_CONFIGURE_ERROR: u8 = 12;
    pub const TRANSITION_ON_CLEANUP_SUCCESS: u8 = 20;
    pub const TRANSITION_ON_CLEANUP_FAILURE: u8 = 21;
    pub const TRANSITION_ON_CLEANUP_ERROR: u8 = 22;
    pub const TRANSITION_ON_ACTIVATE_SUCCESS: u8 = 30;
    pub const TRANSITION_ON_ACTIVATE_FAILURE: u8 = 31;
    pub const TRANSITION_ON_ACTIVATE_ERROR: u8 = 32;
    pub const TRANSITION_ON_DEACTIVATE_SUCCESS: u8 = 40;
    pub const TRANSITION_ON_DEACTIVATE_FAILURE: u8 = 41;
    pub const TRANSITION_ON_DEACTIVATE_ERROR: u8 = 42;
    pub const TRANSITION_ON_SHUTDOWN_SUCCESS: u8 = 50;
    pub const TRANSITION_ON_SHUTDOWN_FAILURE: u8 = 51;
    pub const TRANSITION_ON_SHUTDOWN_ERROR: u8 = 52;
    pub const TRANSITION_ON_ERROR_SUCCESS: u8 = 60;
    pub const TRANSITION_ON_ERROR_FAILURE: u8 = 61;
    pub const TRANSITION_ON_ERROR_ERROR: u8 = 62;
    pub const TRANSITION_CALLBACK_SUCCESS: u8 = 97;
    pub const TRANSITION_CALLBACK_FAILURE: u8 = 98;
    pub const TRANSITION_CALLBACK_ERROR: u8 = 99;
}

impl Default for Transition {
    fn default() -> Self {
        Transition {
            id: 0,
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Transition {}
