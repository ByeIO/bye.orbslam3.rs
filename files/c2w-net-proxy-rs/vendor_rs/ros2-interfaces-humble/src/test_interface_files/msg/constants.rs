use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constants {

}

impl Constants {
    pub const BOOL_CONST: bool = true;
    pub const BYTE_CONST: u8 = 50;
    pub const CHAR_CONST: i8 = 100;
    pub const FLOAT32_CONST: f32 = 1.125;
    pub const FLOAT64_CONST: f64 = 1.125;
    pub const INT8_CONST: i8 = -50;
    pub const UINT8_CONST: u8 = 200;
    pub const INT16_CONST: i16 = -1000;
    pub const UINT16_CONST: u16 = 2000;
    pub const INT32_CONST: i32 = -30000;
    pub const UINT32_CONST: u32 = 60000;
    pub const INT64_CONST: i64 = -40000000;
    pub const UINT64_CONST: u64 = 50000000;
}

impl Default for Constants {
    fn default() -> Self {
        Constants {

        }
    }
}

impl ros2_client::Message for Constants {}
