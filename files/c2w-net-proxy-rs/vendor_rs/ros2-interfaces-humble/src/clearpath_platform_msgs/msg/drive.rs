use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    pub mode: i8,
    pub drivers: [f32; 2],
}

impl Drive {
    pub const MODE_VELOCITY: i8 = 0;
    pub const MODE_PWM: i8 = 1;
    pub const MODE_NONE: i8 = -1;
    pub const LEFT: i8 = 0;
    pub const RIGHT: i8 = 1;
}

impl Default for Drive {
    fn default() -> Self {
        Drive {
            mode: 0,
            drivers: [0.0; 2],
        }
    }
}

impl ros2_client::Message for Drive {}
