use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeftEyeLeds {
    pub colors: [crate::std_msgs::msg::ColorRGBA; 8],
}

impl LeftEyeLeds {
    pub const L0: i32 = 0;
    pub const L1: i32 = 1;
    pub const L2: i32 = 2;
    pub const L3: i32 = 3;
    pub const L4: i32 = 4;
    pub const L5: i32 = 5;
    pub const L6: i32 = 6;
    pub const L7: i32 = 7;
    pub const NUM_LEDS: i32 = 8;
}

impl Default for LeftEyeLeds {
    fn default() -> Self {
        LeftEyeLeds {
            colors: core::array::from_fn(|_| crate::std_msgs::msg::ColorRGBA::default()),
        }
    }
}

impl ros2_client::Message for LeftEyeLeds {}
