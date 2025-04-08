use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    #[serde(rename = "type")]    pub type_: u8,
    pub index: u8,
}

impl Shape {
    pub const NONE: u8 = 0;
    pub const BOX: u8 = 1;
    pub const CIRCLE: u8 = 2;
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            type_: 0,
            index: 0,
        }
    }
}

impl ros2_client::Message for Shape {}
