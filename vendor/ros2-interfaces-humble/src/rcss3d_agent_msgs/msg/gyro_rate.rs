use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GyroRate {
    pub name: ::std::string::String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for GyroRate {
    fn default() -> Self {
        GyroRate {
            name: ::std::string::String::new(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ros2_client::Message for GyroRate {}
