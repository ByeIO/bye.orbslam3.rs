use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gesture {
    pub header: crate::std_msgs::msg::Header,
    pub gesture: u8,
    pub handedness: u8,
}

impl Gesture {
    pub const HANDS_ON_FACE: u8 = 1;
    pub const ARMS_CROSSED: u8 = 2;
    pub const LEFT_HAND_RAISED: u8 = 3;
    pub const RIGHT_HAND_RAISED: u8 = 4;
    pub const BOTH_HANDS_RAISED: u8 = 5;
    pub const WAVING: u8 = 6;
    pub const CLOSED_FIST: u8 = 7;
    pub const OPEN_PALM: u8 = 8;
    pub const POINTING_UP: u8 = 9;
    pub const THUMB_DOWN: u8 = 10;
    pub const THUMB_UP: u8 = 11;
    pub const VICTORY: u8 = 12;
    pub const LOVE: u8 = 13;
    pub const OTHER: u8 = 0;
    pub const RIGHT: u8 = 1;
    pub const LEFT: u8 = 2;
    pub const INDEPENDENT: u8 = 0;
}

impl Default for Gesture {
    fn default() -> Self {
        Gesture {
            header: crate::std_msgs::msg::Header::default(),
            gesture: 0,
            handedness: 0,
        }
    }
}

impl ros2_client::Message for Gesture {}
