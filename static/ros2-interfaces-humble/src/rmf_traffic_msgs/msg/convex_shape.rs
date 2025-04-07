use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConvexShape {
    #[serde(rename = "type")]    pub type_: u8,
    pub index: u8,
}

impl ConvexShape {
    pub const NONE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const CIRCLE: u8 = 2;
}

impl Default for ConvexShape {
    fn default() -> Self {
        ConvexShape {
            type_: 0,
            index: 0,
        }
    }
}

impl ros2_client::Message for ConvexShape {}
