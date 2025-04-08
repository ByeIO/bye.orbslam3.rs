use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionUpdate {
    pub drone_id: ::std::string::String,
    pub mission_id: i32,
    pub item_id: i32,
    pub action: u8,
    pub mission: ::std::string::String,
}

impl MissionUpdate {
    pub const EXECUTE: u8 = 0;
    pub const LOAD: u8 = 1;
    pub const START: u8 = 2;
    pub const PAUSE: u8 = 3;
    pub const RESUME: u8 = 4;
    pub const STOP: u8 = 5;
    pub const JUMP_TO: u8 = 6;
    pub const REPEAT: u8 = 7;
    pub const INSERT: u8 = 8;
    pub const MODIFY: u8 = 9;
    pub const REMOVE: u8 = 10;
    pub const RESET: u8 = 11;
    pub const ABORT: u8 = 12;
}

impl Default for MissionUpdate {
    fn default() -> Self {
        MissionUpdate {
            drone_id: ::std::string::String::new(),
            mission_id: 0,
            item_id: 0,
            action: 0,
            mission: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MissionUpdate {}
