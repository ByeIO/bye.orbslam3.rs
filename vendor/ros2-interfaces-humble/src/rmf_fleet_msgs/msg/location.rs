use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub t: crate::builtin_interfaces::msg::Time,
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
    pub obey_approach_speed_limit: bool, // default: false
    pub approach_speed_limit: f32,
    pub level_name: ::std::string::String,
    pub index: u64,
}

impl Default for Location {
    fn default() -> Self {
        Location {
            t: crate::builtin_interfaces::msg::Time::default(),
            x: 0.0,
            y: 0.0,
            yaw: 0.0,
            obey_approach_speed_limit: false,
            approach_speed_limit: 0.0,
            level_name: ::std::string::String::new(),
            index: 0,
        }
    }
}

impl ros2_client::Message for Location {}
