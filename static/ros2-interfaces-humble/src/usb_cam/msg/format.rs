use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    pub pixel_format: ::std::string::String,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            pixel_format: ::std::string::String::new(),
            width: 0,
            height: 0,
            fps: 0.0,
        }
    }
}

impl ros2_client::Message for Format {}
