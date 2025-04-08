use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargerState {
    pub charger_time: crate::builtin_interfaces::msg::Time,
    pub state: u32,
    pub charger_name: ::std::string::String,
    pub error_message: ::std::string::String,
    pub request_id: ::std::string::String,
    pub robot_fleet: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub time_to_fully_charged: crate::builtin_interfaces::msg::Duration,
}

impl ChargerState {
    pub const CHARGER_IDLE: u32 = 1;
    pub const CHARGER_ASSIGNED: u32 = 2;
    pub const CHARGER_CHARGING: u32 = 3;
    pub const CHARGER_RELEASED: u32 = 4;
    pub const CHARGER_ERROR: u32 = 200;
}

impl Default for ChargerState {
    fn default() -> Self {
        ChargerState {
            charger_time: crate::builtin_interfaces::msg::Time::default(),
            state: 0,
            charger_name: ::std::string::String::new(),
            error_message: ::std::string::String::new(),
            request_id: ::std::string::String::new(),
            robot_fleet: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            time_to_fully_charged: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for ChargerState {}
