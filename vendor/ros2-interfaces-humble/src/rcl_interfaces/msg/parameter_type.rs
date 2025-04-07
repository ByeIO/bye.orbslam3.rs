use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterType {

}

impl ParameterType {
    pub const PARAMETER_NOT_SET: u8 = 0;
    pub const PARAMETER_BOOL: u8 = 1;
    pub const PARAMETER_INTEGER: u8 = 2;
    pub const PARAMETER_DOUBLE: u8 = 3;
    pub const PARAMETER_STRING: u8 = 4;
    pub const PARAMETER_BYTE_ARRAY: u8 = 5;
    pub const PARAMETER_BOOL_ARRAY: u8 = 6;
    pub const PARAMETER_INTEGER_ARRAY: u8 = 7;
    pub const PARAMETER_DOUBLE_ARRAY: u8 = 8;
    pub const PARAMETER_STRING_ARRAY: u8 = 9;
}

impl Default for ParameterType {
    fn default() -> Self {
        ParameterType {

        }
    }
}

impl ros2_client::Message for ParameterType {}
