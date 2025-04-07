use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayTypes {
    pub data_int8_static: [i8; 3],
    pub data_bool_static: [bool; 2],
    pub data_int8_unbounded_dynamic: Vec<i8>,
    pub data_bool_unbounded_dynamic: Vec<bool>,
    pub data_int8_bounded_dynamic: Vec<i8>,
    pub data_bool_bounded_dynamic: Vec<bool>,
    pub data_string: ::std::string::String,
    pub data_string_bounded: ::std::string::String,
    pub data_wstring: ::std::string::String,
    pub data_wstring_bounded: ::std::string::String,
}

impl Default for ArrayTypes {
    fn default() -> Self {
        ArrayTypes {
            data_int8_static: [0; 3],
            data_bool_static: [false; 2],
            data_int8_unbounded_dynamic: Vec::new(),
            data_bool_unbounded_dynamic: Vec::new(),
            data_int8_bounded_dynamic: Vec::new(),
            data_bool_bounded_dynamic: Vec::new(),
            data_string: ::std::string::String::new(),
            data_string_bounded: ::std::string::String::new(),
            data_wstring: ::std::string::String::new(),
            data_wstring_bounded: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ArrayTypes {}
