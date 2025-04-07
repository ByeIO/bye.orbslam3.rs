use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertEvent {
    pub alert: i8,
    pub description: ::std::string::String,
}

impl AlertEvent {
    pub const KILL_SWITCH: i8 = -1;
    pub const EMERGENCY_HOVER: i8 = -2;
    pub const EMERGENCY_LAND: i8 = -3;
    pub const INFO_ALERT: i8 = 0;
    pub const FORCE_HOVER: i8 = 1;
    pub const FORCE_LAND: i8 = 2;
}

impl Default for AlertEvent {
    fn default() -> Self {
        AlertEvent {
            alert: 0,
            description: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AlertEvent {}
