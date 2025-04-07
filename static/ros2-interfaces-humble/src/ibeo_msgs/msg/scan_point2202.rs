use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanPoint2202 {
    pub layer: u8,
    pub echo: u8,
    pub transparent_point: bool,
    pub clutter_atmospheric: bool,
    pub ground: bool,
    pub dirt: bool,
    pub horizontal_angle: i16,
    pub radial_distance: u16,
    pub echo_pulse_width: u16,
}

impl Default for ScanPoint2202 {
    fn default() -> Self {
        ScanPoint2202 {
            layer: 0,
            echo: 0,
            transparent_point: false,
            clutter_atmospheric: false,
            ground: false,
            dirt: false,
            horizontal_angle: 0,
            radial_distance: 0,
            echo_pulse_width: 0,
        }
    }
}

impl ros2_client::Message for ScanPoint2202 {}
