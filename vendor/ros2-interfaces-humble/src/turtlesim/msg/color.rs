use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl ros2_client::Message for Color {}
