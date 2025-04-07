use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Velocity2D {
    pub velocity_x: i16,
    pub velocity_y: i16,
}

impl Default for Velocity2D {
    fn default() -> Self {
        Velocity2D {
            velocity_x: 0,
            velocity_y: 0,
        }
    }
}

impl ros2_client::Message for Velocity2D {}
