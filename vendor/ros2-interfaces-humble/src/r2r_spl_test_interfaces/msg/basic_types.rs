use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicTypes {
    pub val_bool: bool,
    pub val_byte: u8,
    pub val_char: i8,
    pub val_float32: f32,
    pub val_float64: f64,
    pub val_int8: i8,
    pub val_uint8: u8,
    pub val_int16: i16,
    pub val_uint16: u16,
    pub val_int32: i32,
    pub val_uint32: u32,
    pub val_int64: i64,
    pub val_uint64: u64,
}

impl Default for BasicTypes {
    fn default() -> Self {
        BasicTypes {
            val_bool: false,
            val_byte: 0,
            val_char: 0,
            val_float32: 0.0,
            val_float64: 0.0,
            val_int8: 0,
            val_uint8: 0,
            val_int16: 0,
            val_uint16: 0,
            val_int32: 0,
            val_uint32: 0,
            val_int64: 0,
            val_uint64: 0,
        }
    }
}

impl ros2_client::Message for BasicTypes {}
