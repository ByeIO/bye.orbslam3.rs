use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorSessions {
    pub door_name: ::std::string::String,
    pub sessions: Vec<crate::rmf_door_msgs::msg::Session>,
}

impl Default for DoorSessions {
    fn default() -> Self {
        DoorSessions {
            door_name: ::std::string::String::new(),
            sessions: Vec::new(),
        }
    }
}

impl ros2_client::Message for DoorSessions {}
