use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NAVOdomVelocity {
    pub vel_x: f32,
    pub vel_y: f32,
    pub omega: f32,
    pub timestamp: u32,
    pub coordbase: u8,
}

impl Default for NAVOdomVelocity {
    fn default() -> Self {
        NAVOdomVelocity {
            vel_x: 0.0,
            vel_y: 0.0,
            omega: 0.0,
            timestamp: 0,
            coordbase: 0,
        }
    }
}

impl ros2_client::Message for NAVOdomVelocity {}
