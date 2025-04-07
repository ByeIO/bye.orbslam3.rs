use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub macattr: ::std::string::String,
    pub essid: ::std::string::String,
    pub channel: i32,
    pub rssi: i32,
    pub noise: i32,
    pub beacon: i32,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            macattr: ::std::string::String::new(),
            essid: ::std::string::String::new(),
            channel: 0,
            rssi: 0,
            noise: 0,
            beacon: 0,
        }
    }
}

impl ros2_client::Message for Network {}
