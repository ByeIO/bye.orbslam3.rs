use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightEyeLeds {
    pub colors: [crate::std_msgs::msg::ColorRGBA; 8],
}

impl RightEyeLeds {
    pub const R0: i32 = 0;
    pub const R1: i32 = 1;
    pub const R2: i32 = 2;
    pub const R3: i32 = 3;
    pub const R4: i32 = 4;
    pub const R5: i32 = 5;
    pub const R6: i32 = 6;
    pub const R7: i32 = 7;
    pub const NUM_LEDS: i32 = 8;
}

impl Default for RightEyeLeds {
    fn default() -> Self {
        RightEyeLeds {
            colors: core::array::from_fn(|_| crate::std_msgs::msg::ColorRGBA::default()),
        }
    }
}

impl ros2_client::Message for RightEyeLeds {}
