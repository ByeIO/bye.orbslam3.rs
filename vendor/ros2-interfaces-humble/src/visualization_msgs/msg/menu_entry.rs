use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuEntry {
    pub id: u32,
    pub parent_id: u32,
    pub title: ::std::string::String,
    pub command: ::std::string::String,
    pub command_type: u8,
}

impl MenuEntry {
    pub const FEEDBACK: u8 = 0;
    pub const ROSRUN: u8 = 1;
    pub const ROSLAUNCH: u8 = 2;
}

impl Default for MenuEntry {
    fn default() -> Self {
        MenuEntry {
            id: 0,
            parent_id: 0,
            title: ::std::string::String::new(),
            command: ::std::string::String::new(),
            command_type: 0,
        }
    }
}

impl ros2_client::Message for MenuEntry {}
