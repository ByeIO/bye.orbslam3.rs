use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub id: u8,
    pub label: ::std::string::String,
}

impl State {
    pub const PRIMARY_STATE_UNKNOWN: u8 = 0;
    pub const PRIMARY_STATE_UNCONFIGURED: u8 = 1;
    pub const PRIMARY_STATE_INACTIVE: u8 = 2;
    pub const PRIMARY_STATE_ACTIVE: u8 = 3;
    pub const PRIMARY_STATE_FINALIZED: u8 = 4;
    pub const TRANSITION_STATE_CONFIGURING: u8 = 10;
    pub const TRANSITION_STATE_CLEANINGUP: u8 = 11;
    pub const TRANSITION_STATE_SHUTTINGDOWN: u8 = 12;
    pub const TRANSITION_STATE_ACTIVATING: u8 = 13;
    pub const TRANSITION_STATE_DEACTIVATING: u8 = 14;
    pub const TRANSITION_STATE_ERRORPROCESSING: u8 = 15;
}

impl Default for State {
    fn default() -> Self {
        State {
            id: 0,
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for State {}
