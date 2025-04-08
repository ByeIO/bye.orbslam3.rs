use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PtzState {
    pub mode: i8,
    pub pan: f32,
    pub tilt: f32,
    pub zoom: f32,
}

impl PtzState {
    pub const MODE_IDLE: i8 = 0;
    pub const MODE_POSITION: i8 = 1;
    pub const MODE_VELOCITY: i8 = 2;
}

impl Default for PtzState {
    fn default() -> Self {
        PtzState {
            mode: 0,
            pan: 0.0,
            tilt: 0.0,
            zoom: 0.0,
        }
    }
}

impl ros2_client::Message for PtzState {}
