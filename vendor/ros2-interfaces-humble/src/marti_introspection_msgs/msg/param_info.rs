use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: ::std::string::String,
    pub resolved_name: ::std::string::String,
    pub description: ::std::string::String,
    pub group: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub dynamic: bool,
    pub default_int: i32,
    pub default_float: f32,
    pub default_double: f64,
    pub default_string: ::std::string::String,
    pub default_bool: bool,
    pub max_value: f64,
    pub min_value: f64,
}

impl ParamInfo {
    pub const TYPE_XMLRPC: u8 = 0;
    pub const TYPE_DOUBLE: u8 = 1;
    pub const TYPE_STRING: u8 = 2;
    pub const TYPE_INT: u8 = 3;
    pub const TYPE_FLOAT: u8 = 4;
    pub const TYPE_BOOL: u8 = 5;
}

impl Default for ParamInfo {
    fn default() -> Self {
        ParamInfo {
            name: ::std::string::String::new(),
            resolved_name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            group: ::std::string::String::new(),
            type_: 0,
            dynamic: false,
            default_int: 0,
            default_float: 0.0,
            default_double: 0.0,
            default_string: ::std::string::String::new(),
            default_bool: false,
            max_value: 0.0,
            min_value: 0.0,
        }
    }
}

impl ros2_client::Message for ParamInfo {}
