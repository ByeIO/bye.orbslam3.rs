use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelEncoder {
    pub frequency: f64,
    pub directional: bool,
    pub id: u8,
}

impl Default for WheelEncoder {
    fn default() -> Self {
        WheelEncoder {
            frequency: 0.0,
            directional: false,
            id: 0,
        }
    }
}

impl ros2_client::Message for WheelEncoder {}
