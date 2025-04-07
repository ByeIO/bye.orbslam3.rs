use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftState {
    pub lift_time: crate::builtin_interfaces::msg::Time,
    pub lift_name: ::std::string::String,
    pub available_floors: Vec<::std::string::String>,
    pub current_floor: ::std::string::String,
    pub destination_floor: ::std::string::String,
    pub door_state: u8,
    pub motion_state: u8,
    pub available_modes: Vec<u8>,
    pub current_mode: u8,
    pub session_id: ::std::string::String,
}

impl LiftState {
    pub const DOOR_CLOSED: u8 = 0;
    pub const DOOR_MOVING: u8 = 1;
    pub const DOOR_OPEN: u8 = 2;
    pub const MOTION_STOPPED: u8 = 0;
    pub const MOTION_UP: u8 = 1;
    pub const MOTION_DOWN: u8 = 2;
    pub const MOTION_UNKNOWN: u8 = 3;
    pub const MODE_UNKNOWN: u8 = 0;
    pub const MODE_HUMAN: u8 = 1;
    pub const MODE_AGV: u8 = 2;
    pub const MODE_FIRE: u8 = 3;
    pub const MODE_OFFLINE: u8 = 4;
    pub const MODE_EMERGENCY: u8 = 5;
}

impl Default for LiftState {
    fn default() -> Self {
        LiftState {
            lift_time: crate::builtin_interfaces::msg::Time::default(),
            lift_name: ::std::string::String::new(),
            available_floors: Vec::new(),
            current_floor: ::std::string::String::new(),
            destination_floor: ::std::string::String::new(),
            door_state: 0,
            motion_state: 0,
            available_modes: Vec::new(),
            current_mode: 0,
            session_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LiftState {}
