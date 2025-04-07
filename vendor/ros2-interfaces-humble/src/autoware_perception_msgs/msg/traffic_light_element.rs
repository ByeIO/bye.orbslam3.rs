use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficLightElement {
    pub color: u8,
    pub shape: u8,
    pub status: u8,
    pub confidence: f32,
}

impl TrafficLightElement {
    pub const UNKNOWN: u8 = 0;
    pub const RED: u8 = 1;
    pub const AMBER: u8 = 2;
    pub const GREEN: u8 = 3;
    pub const WHITE: u8 = 4;
    pub const CIRCLE: u8 = 1;
    pub const LEFT_ARROW: u8 = 2;
    pub const RIGHT_ARROW: u8 = 3;
    pub const UP_ARROW: u8 = 4;
    pub const UP_LEFT_ARROW: u8 = 5;
    pub const UP_RIGHT_ARROW: u8 = 6;
    pub const DOWN_ARROW: u8 = 7;
    pub const DOWN_LEFT_ARROW: u8 = 8;
    pub const DOWN_RIGHT_ARROW: u8 = 9;
    pub const CROSS: u8 = 10;
    pub const SOLID_OFF: u8 = 1;
    pub const SOLID_ON: u8 = 2;
    pub const FLASHING: u8 = 3;
}

impl Default for TrafficLightElement {
    fn default() -> Self {
        TrafficLightElement {
            color: 0,
            shape: 0,
            status: 0,
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for TrafficLightElement {}
