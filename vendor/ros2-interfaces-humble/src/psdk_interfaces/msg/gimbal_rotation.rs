use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalRotation {
    pub payload_index: u8, // default: 1
    pub rotation_mode: u8,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub time: f32,
}

impl Default for GimbalRotation {
    fn default() -> Self {
        GimbalRotation {
            payload_index: 1,
            rotation_mode: 0,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            time: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalRotation {}
