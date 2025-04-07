use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyPosture {
    pub header: crate::std_msgs::msg::Header,
    pub posture: u8,
}

impl BodyPosture {
    pub const STANDING: u8 = 1;
    pub const SITTING: u8 = 2;
    pub const CROUCHING: u8 = 3;
    pub const LAYING: u8 = 4;
    pub const OTHER: u8 = 0;
}

impl Default for BodyPosture {
    fn default() -> Self {
        BodyPosture {
            header: crate::std_msgs::msg::Header::default(),
            posture: 0,
        }
    }
}

impl ros2_client::Message for BodyPosture {}
