use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadLeds {
    pub intensities: [f32; 12],
}

impl HeadLeds {
    pub const B0: i32 = 0;
    pub const B1: i32 = 1;
    pub const B2: i32 = 2;
    pub const B3: i32 = 3;
    pub const B4: i32 = 4;
    pub const B5: i32 = 5;
    pub const B6: i32 = 6;
    pub const B7: i32 = 7;
    pub const B8: i32 = 8;
    pub const B9: i32 = 9;
    pub const B10: i32 = 10;
    pub const B11: i32 = 11;
    pub const NUM_LEDS: i32 = 12;
}

impl Default for HeadLeds {
    fn default() -> Self {
        HeadLeds {
            intensities: [0.0; 12],
        }
    }
}

impl ros2_client::Message for HeadLeds {}
