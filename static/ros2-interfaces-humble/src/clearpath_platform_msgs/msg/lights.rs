use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lights {
    pub lights: Vec<crate::clearpath_platform_msgs::msg::RGB>,
}

impl Lights {
    pub const D100_LIGHTS_REAR_LEFT: u8 = 0;
    pub const D100_LIGHTS_FRONT_LEFT: u8 = 1;
    pub const D100_LIGHTS_FRONT_RIGHT: u8 = 2;
    pub const D100_LIGHTS_REAR_RIGHT: u8 = 3;
    pub const D150_LIGHTS_REAR_LEFT: u8 = 0;
    pub const D150_LIGHTS_FRONT_LEFT: u8 = 1;
    pub const D150_LIGHTS_FRONT_RIGHT: u8 = 2;
    pub const D150_LIGHTS_REAR_RIGHT: u8 = 3;
    pub const R100_LIGHTS_FRONT_PORT_UPPER: u8 = 0;
    pub const R100_LIGHTS_FRONT_PORT_LOWER: u8 = 1;
    pub const R100_LIGHTS_FRONT_STARBOARD_UPPER: u8 = 2;
    pub const R100_LIGHTS_FRONT_STARBOARD_LOWER: u8 = 3;
    pub const R100_LIGHTS_REAR_PORT_UPPER: u8 = 4;
    pub const R100_LIGHTS_REAR_PORT_LOWER: u8 = 5;
    pub const R100_LIGHTS_REAR_STARBOARD_UPPER: u8 = 6;
    pub const R100_LIGHTS_REAR_STARBOARD_LOWER: u8 = 7;
    pub const W200_LIGHTS_FRONT_LEFT: u8 = 0;
    pub const W200_LIGHTS_FRONT_RIGHT: u8 = 1;
    pub const W200_LIGHTS_REAR_LEFT: u8 = 2;
    pub const W200_LIGHTS_REAR_RIGHT: u8 = 3;
}

impl Default for Lights {
    fn default() -> Self {
        Lights {
            lights: Vec::new(),
        }
    }
}

impl ros2_client::Message for Lights {}
