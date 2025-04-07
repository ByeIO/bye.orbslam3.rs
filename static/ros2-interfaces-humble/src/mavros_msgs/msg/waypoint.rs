use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    pub frame: u8,
    pub command: u16,
    pub is_current: bool,
    pub autocontinue: bool,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x_lat: f64,
    pub y_long: f64,
    pub z_alt: f64,
}

impl Waypoint {
    pub const FRAME_GLOBAL: u8 = 0;
    pub const FRAME_LOCAL_NED: u8 = 1;
    pub const FRAME_MISSION: u8 = 2;
    pub const FRAME_GLOBAL_REL_ALT: u8 = 3;
    pub const FRAME_LOCAL_ENU: u8 = 4;
    pub const FRAME_GLOBAL_INT: u8 = 5;
    pub const FRAME_GLOBAL_RELATIVE_ALT_INT: u8 = 6;
    pub const FRAME_LOCAL_OFFSET_NED: u8 = 7;
    pub const FRAME_BODY_NED: u8 = 8;
    pub const FRAME_BODY_OFFSET_NED: u8 = 9;
    pub const FRAME_GLOBAL_TERRAIN_ALT: u8 = 10;
    pub const FRAME_GLOBAL_TERRAIN_ALT_INT: u8 = 11;
    pub const FRAME_BODY_FRD: u8 = 12;
    pub const FRAME_RESERVED_13: u8 = 13;
    pub const FRAME_RESERVED_14: u8 = 14;
    pub const FRAME_RESERVED_15: u8 = 15;
    pub const FRAME_RESERVED_16: u8 = 16;
    pub const FRAME_RESERVED_17: u8 = 17;
    pub const FRAME_RESERVED_18: u8 = 18;
    pub const FRAME_RESERVED_19: u8 = 19;
    pub const FRAME_LOCAL_FRD: u8 = 20;
    pub const FRAME_LOCAL_FLU: u8 = 21;
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint {
            frame: 0,
            command: 0,
            is_current: false,
            autocontinue: false,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            x_lat: 0.0,
            y_long: 0.0,
            z_alt: 0.0,
        }
    }
}

impl ros2_client::Message for Waypoint {}
