use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftRequest {
    pub lift_name: ::std::string::String,
    pub request_time: crate::builtin_interfaces::msg::Time,
    pub session_id: ::std::string::String,
    pub request_type: u8,
    pub destination_floor: ::std::string::String,
    pub door_state: u8,
}

impl LiftRequest {
    pub const REQUEST_END_SESSION: u8 = 0;
    pub const REQUEST_AGV_MODE: u8 = 1;
    pub const REQUEST_HUMAN_MODE: u8 = 2;
    pub const DOOR_CLOSED: u8 = 0;
    pub const DOOR_OPEN: u8 = 2;
}

impl Default for LiftRequest {
    fn default() -> Self {
        LiftRequest {
            lift_name: ::std::string::String::new(),
            request_time: crate::builtin_interfaces::msg::Time::default(),
            session_id: ::std::string::String::new(),
            request_type: 0,
            destination_floor: ::std::string::String::new(),
            door_state: 0,
        }
    }
}

impl ros2_client::Message for LiftRequest {}
