use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointField {
    pub name: ::std::string::String,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

impl PointField {
    pub const INT8: u8 = 1;
    pub const UINT8: u8 = 2;
    pub const INT16: u8 = 3;
    pub const UINT16: u8 = 4;
    pub const INT32: u8 = 5;
    pub const UINT32: u8 = 6;
    pub const FLOAT32: u8 = 7;
    pub const FLOAT64: u8 = 8;
}

impl Default for PointField {
    fn default() -> Self {
        PointField {
            name: ::std::string::String::new(),
            offset: 0,
            datatype: 0,
            count: 0,
        }
    }
}

impl ros2_client::Message for PointField {}
