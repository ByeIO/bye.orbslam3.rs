use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugValue {
    pub header: crate::std_msgs::msg::Header,
    pub index: i32,
    pub array_id: i32,
    pub name: ::std::string::String,
    pub value_float: f32,
    pub value_int: i32,
    pub data: Vec<f32>,
    #[serde(rename = "type")]    pub type_: u8,
}

impl DebugValue {
    pub const TYPE_DEBUG: u8 = 0;
    pub const TYPE_DEBUG_VECT: u8 = 1;
    pub const TYPE_DEBUG_FLOAT_ARRAY: u8 = 2;
    pub const TYPE_NAMED_VALUE_FLOAT: u8 = 3;
    pub const TYPE_NAMED_VALUE_INT: u8 = 4;
}

impl Default for DebugValue {
    fn default() -> Self {
        DebugValue {
            header: crate::std_msgs::msg::Header::default(),
            index: 0,
            array_id: 0,
            name: ::std::string::String::new(),
            value_float: 0.0,
            value_int: 0,
            data: Vec::new(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for DebugValue {}
