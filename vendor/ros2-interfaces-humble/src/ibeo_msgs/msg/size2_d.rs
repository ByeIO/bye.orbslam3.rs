use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Size2D {
    pub size_x: u16,
    pub size_y: u16,
}

impl Default for Size2D {
    fn default() -> Self {
        Size2D {
            size_x: 0,
            size_y: 0,
        }
    }
}

impl ros2_client::Message for Size2D {}
