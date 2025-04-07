use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchCommand {
    pub fleet_name: ::std::string::String,
    pub task_id: ::std::string::String,
    pub dispatch_id: u64,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    #[serde(rename = "type")]    pub type_: u8,
}

impl DispatchCommand {
    pub const TYPE_AWARD: u8 = 1;
    pub const TYPE_REMOVE: u8 = 2;
}

impl Default for DispatchCommand {
    fn default() -> Self {
        DispatchCommand {
            fleet_name: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
            dispatch_id: 0,
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for DispatchCommand {}
