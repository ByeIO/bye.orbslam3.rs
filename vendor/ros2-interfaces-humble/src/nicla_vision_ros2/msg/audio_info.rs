use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioInfo {
    pub channels: u8,
    pub sample_rate: u32,
    pub sample_format: ::std::string::String,
    pub bitrate: u32,
    pub coding_format: ::std::string::String,
}

impl Default for AudioInfo {
    fn default() -> Self {
        AudioInfo {
            channels: 0,
            sample_rate: 0,
            sample_format: ::std::string::String::new(),
            bitrate: 0,
            coding_format: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AudioInfo {}
