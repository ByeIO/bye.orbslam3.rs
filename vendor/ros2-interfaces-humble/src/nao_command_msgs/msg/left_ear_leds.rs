use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeftEarLeds {
    pub intensities: [f32; 10],
}

impl LeftEarLeds {
    pub const L0: i32 = 0;
    pub const L1: i32 = 1;
    pub const L2: i32 = 2;
    pub const L3: i32 = 3;
    pub const L4: i32 = 4;
    pub const L5: i32 = 5;
    pub const L6: i32 = 6;
    pub const L7: i32 = 7;
    pub const L8: i32 = 8;
    pub const L9: i32 = 9;
    pub const NUM_LEDS: i32 = 10;
}

impl Default for LeftEarLeds {
    fn default() -> Self {
        LeftEarLeds {
            intensities: [0.0; 10],
        }
    }
}

impl ros2_client::Message for LeftEarLeds {}
