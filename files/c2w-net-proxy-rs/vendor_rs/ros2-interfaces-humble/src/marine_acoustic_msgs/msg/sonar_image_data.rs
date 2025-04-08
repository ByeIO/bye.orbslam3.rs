use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonarImageData {
    pub is_bigendian: bool,
    pub dtype: u32,
    pub beam_count: u32,
    pub data: Vec<u8>,
}

impl SonarImageData {
    pub const DTYPE_UINT8: u32 = 0;
    pub const DTYPE_INT8: u32 = 1;
    pub const DTYPE_UINT16: u32 = 2;
    pub const DTYPE_INT16: u32 = 3;
    pub const DTYPE_UINT32: u32 = 4;
    pub const DTYPE_INT32: u32 = 5;
    pub const DTYPE_UINT64: u32 = 6;
    pub const DTYPE_INT64: u32 = 7;
    pub const DTYPE_FLOAT32: u32 = 8;
    pub const DTYPE_FLOAT64: u32 = 9;
}

impl Default for SonarImageData {
    fn default() -> Self {
        SonarImageData {
            is_bigendian: false,
            dtype: 0,
            beam_count: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for SonarImageData {}
