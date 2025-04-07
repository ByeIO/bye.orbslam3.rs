use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorRequest {
    pub request_time: crate::builtin_interfaces::msg::Time,
    pub requester_id: ::std::string::String,
    pub door_name: ::std::string::String,
    pub requested_mode: crate::rmf_door_msgs::msg::DoorMode,
}

impl Default for DoorRequest {
    fn default() -> Self {
        DoorRequest {
            request_time: crate::builtin_interfaces::msg::Time::default(),
            requester_id: ::std::string::String::new(),
            door_name: ::std::string::String::new(),
            requested_mode: crate::rmf_door_msgs::msg::DoorMode::default(),
        }
    }
}

impl ros2_client::Message for DoorRequest {}
