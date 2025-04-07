use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub bitrate: f32,
    pub txpower: i16,
    pub link_quality_raw: ::std::string::String,
    pub link_quality: f32,
    pub signal_level: i16,
    pub noise_level: i16,
    pub essid: ::std::string::String,
    pub bssid: ::std::string::String,
    pub frequency: f64,
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            bitrate: 0.0,
            txpower: 0,
            link_quality_raw: ::std::string::String::new(),
            link_quality: 0.0,
            signal_level: 0,
            noise_level: 0,
            essid: ::std::string::String::new(),
            bssid: ::std::string::String::new(),
            frequency: 0.0,
        }
    }
}

impl ros2_client::Message for Connection {}
