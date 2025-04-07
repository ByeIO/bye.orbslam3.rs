use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightEarLeds {
    pub intensities: [f32; 10],
}

impl RightEarLeds {
    pub const R0: i32 = 0;
    pub const R1: i32 = 1;
    pub const R2: i32 = 2;
    pub const R3: i32 = 3;
    pub const R4: i32 = 4;
    pub const R5: i32 = 5;
    pub const R6: i32 = 6;
    pub const R7: i32 = 7;
    pub const R8: i32 = 8;
    pub const R9: i32 = 9;
    pub const NUM_LEDS: i32 = 10;
}

impl Default for RightEarLeds {
    fn default() -> Self {
        RightEarLeds {
            intensities: [0.0; 10],
        }
    }
}

impl ros2_client::Message for RightEarLeds {}
