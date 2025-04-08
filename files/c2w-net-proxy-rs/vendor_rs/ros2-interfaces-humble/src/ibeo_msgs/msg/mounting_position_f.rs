use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountingPositionF {
    pub yaw_angle: f32,
    pub pitch_angle: f32,
    pub roll_angle: f32,
    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,
}

impl Default for MountingPositionF {
    fn default() -> Self {
        MountingPositionF {
            yaw_angle: 0.0,
            pitch_angle: 0.0,
            roll_angle: 0.0,
            x_position: 0.0,
            y_position: 0.0,
            z_position: 0.0,
        }
    }
}

impl ros2_client::Message for MountingPositionF {}
