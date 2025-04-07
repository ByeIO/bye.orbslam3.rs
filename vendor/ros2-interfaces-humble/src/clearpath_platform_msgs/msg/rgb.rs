use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for RGB {
    fn default() -> Self {
        RGB {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl ros2_client::Message for RGB {}
