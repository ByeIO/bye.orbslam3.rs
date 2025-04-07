use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanPoint2205 {
    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,
    pub echo_width: f32,
    pub device_id: u8,
    pub layer: u8,
    pub echo: u8,
    pub time_offset: u32,
    pub ground: bool,
    pub dirt: bool,
    pub precipitation: bool,
    pub transparent: bool,
}

impl Default for ScanPoint2205 {
    fn default() -> Self {
        ScanPoint2205 {
            x_position: 0.0,
            y_position: 0.0,
            z_position: 0.0,
            echo_width: 0.0,
            device_id: 0,
            layer: 0,
            echo: 0,
            time_offset: 0,
            ground: false,
            dirt: false,
            precipitation: false,
            transparent: false,
        }
    }
}

impl ros2_client::Message for ScanPoint2205 {}
