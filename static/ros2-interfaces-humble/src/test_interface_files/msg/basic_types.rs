use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicTypes {
    pub bool_value: bool,
    pub byte_value: u8,
    pub char_value: i8,
    pub float32_value: f32,
    pub float64_value: f64,
    pub int8_value: i8,
    pub uint8_value: u8,
    pub int16_value: i16,
    pub uint16_value: u16,
    pub int32_value: i32,
    pub uint32_value: u32,
    pub int64_value: i64,
    pub uint64_value: u64,
}

impl Default for BasicTypes {
    fn default() -> Self {
        BasicTypes {
            bool_value: false,
            byte_value: 0,
            char_value: 0,
            float32_value: 0.0,
            float64_value: 0.0,
            int8_value: 0,
            uint8_value: 0,
            int16_value: 0,
            uint16_value: 0,
            int32_value: 0,
            uint32_value: 0,
            int64_value: 0,
            uint64_value: 0,
        }
    }
}

impl ros2_client::Message for BasicTypes {}
