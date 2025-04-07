use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneBoundary {
    pub style: u8,
    pub color: u8,
    pub line: Vec<crate::geometry_msgs::msg::Point>,
}

impl LaneBoundary {
    pub const UNKNOWN: u8 = 0;
    pub const SOLID: u8 = 1;
    pub const DASHED: u8 = 2;
    pub const SOLID_DASHED: u8 = 3;
    pub const DASHED_SOLID: u8 = 4;
    pub const SOLID_SOLID: u8 = 5;
    pub const WHITE: u8 = 1;
    pub const YELLOW: u8 = 2;
}

impl Default for LaneBoundary {
    fn default() -> Self {
        LaneBoundary {
            style: 0,
            color: 0,
            line: Vec::new(),
        }
    }
}

impl ros2_client::Message for LaneBoundary {}
