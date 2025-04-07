use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArray {
    pub bools: Vec<bool>,
    pub uint8s: Vec<u8>,
    pub uint16s: [u16; 32],
    pub uint32s: Vec<u32>,
    pub uint64s: Vec<u64>,
    pub int8s: Vec<i8>,
    pub int16s: Vec<i16>,
    pub int32s: Vec<i32>,
    pub int64s: [i64; 32],
    pub float32s: Vec<f32>,
    pub float64s: Vec<f64>,
    pub times: Vec<crate::builtin_interfaces::msg::Time>,
    pub durations: [crate::builtin_interfaces::msg::Duration; 12],
    pub strings: Vec<::std::string::String>,
    pub subarrays_fixed: [crate::ros_babel_fish_test_msgs::msg::TestSubArray; 10],
    pub subarrays: Vec<crate::ros_babel_fish_test_msgs::msg::TestSubArray>,
}

impl Default for TestArray {
    fn default() -> Self {
        TestArray {
            bools: Vec::new(),
            uint8s: Vec::new(),
            uint16s: [0; 32],
            uint32s: Vec::new(),
            uint64s: Vec::new(),
            int8s: Vec::new(),
            int16s: Vec::new(),
            int32s: Vec::new(),
            int64s: [0; 32],
            float32s: Vec::new(),
            float64s: Vec::new(),
            times: Vec::new(),
            durations: core::array::from_fn(|_| crate::builtin_interfaces::msg::Duration::default()),
            strings: Vec::new(),
            subarrays_fixed: core::array::from_fn(|_| crate::ros_babel_fish_test_msgs::msg::TestSubArray::default()),
            subarrays: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestArray {}
