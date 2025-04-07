use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorState {
    pub door_time: crate::builtin_interfaces::msg::Time,
    pub door_name: ::std::string::String,
    pub current_mode: crate::rmf_door_msgs::msg::DoorMode,
}

impl Default for DoorState {
    fn default() -> Self {
        DoorState {
            door_time: crate::builtin_interfaces::msg::Time::default(),
            door_name: ::std::string::String::new(),
            current_mode: crate::rmf_door_msgs::msg::DoorMode::default(),
        }
    }
}

impl ros2_client::Message for DoorState {}
