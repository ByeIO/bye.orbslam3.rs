use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandingTarget {
    pub header: crate::std_msgs::msg::Header,
    pub target_num: u8,
    pub frame: u8,
    pub angle: [f32; 2],
    pub distance: f32,
    pub size: [f32; 2],
    pub pose: crate::geometry_msgs::msg::Pose,
    #[serde(rename = "type")]    pub type_: u8,
}

impl LandingTarget {
    pub const GLOBAL: u8 = 0;
    pub const LOCAL_NED: u8 = 2;
    pub const MISSION: u8 = 3;
    pub const GLOBAL_RELATIVE_ALT: u8 = 4;
    pub const LOCAL_ENU: u8 = 5;
    pub const GLOBAL_INT: u8 = 6;
    pub const GLOBAL_RELATIVE_ALT_INT: u8 = 7;
    pub const LOCAL_OFFSET_NED: u8 = 8;
    pub const BODY_NED: u8 = 9;
    pub const BODY_OFFSET_NED: u8 = 10;
    pub const GLOBAL_TERRAIN_ALT: u8 = 11;
    pub const GLOBAL_TERRAIN_ALT_INT: u8 = 12;
    pub const LIGHT_BEACON: u8 = 0;
    pub const RADIO_BEACON: u8 = 1;
    pub const VISION_FIDUCIAL: u8 = 2;
    pub const VISION_OTHER: u8 = 3;
}

impl Default for LandingTarget {
    fn default() -> Self {
        LandingTarget {
            header: crate::std_msgs::msg::Header::default(),
            target_num: 0,
            frame: 0,
            angle: [0.0; 2],
            distance: 0.0,
            size: [0.0; 2],
            pose: crate::geometry_msgs::msg::Pose::default(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for LandingTarget {}
