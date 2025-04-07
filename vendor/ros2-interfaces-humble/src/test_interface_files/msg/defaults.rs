use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defaults {
    pub bool_value: bool, // default: true
    pub byte_value: u8, // default: 50
    pub char_value: i8, // default: 100
    pub float32_value: f32, // default: 1.125
    pub float64_value: f64, // default: 1.125
    pub int8_value: i8, // default: -50
    pub uint8_value: u8, // default: 200
    pub int16_value: i16, // default: -1000
    pub uint16_value: u16, // default: 2000
    pub int32_value: i32, // default: -30000
    pub uint32_value: u32, // default: 60000
    pub int64_value: i64, // default: -40000000
    pub uint64_value: u64, // default: 50000000
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            bool_value: true,
            byte_value: 50,
            char_value: 100,
            float32_value: 1.125,
            float64_value: 1.125,
            int8_value: -50,
            uint8_value: 200,
            int16_value: -1000,
            uint16_value: 2000,
            int32_value: -30000,
            uint32_value: 60000,
            int64_value: -40000000,
            uint64_value: 50000000,
        }
    }
}

impl ros2_client::Message for Defaults {}
