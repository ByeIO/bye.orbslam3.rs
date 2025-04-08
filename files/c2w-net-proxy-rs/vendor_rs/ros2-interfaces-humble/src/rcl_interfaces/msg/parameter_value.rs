use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    #[serde(rename = "type")]    pub type_: u8,
    pub bool_value: bool,
    pub integer_value: i64,
    pub double_value: f64,
    pub string_value: ::std::string::String,
    pub byte_array_value: Vec<u8>,
    pub bool_array_value: Vec<bool>,
    pub integer_array_value: Vec<i64>,
    pub double_array_value: Vec<f64>,
    pub string_array_value: Vec<::std::string::String>,
}

impl Default for ParameterValue {
    fn default() -> Self {
        ParameterValue {
            type_: 0,
            bool_value: false,
            integer_value: 0,
            double_value: 0.0,
            string_value: ::std::string::String::new(),
            byte_array_value: Vec::new(),
            bool_array_value: Vec::new(),
            integer_array_value: Vec::new(),
            double_array_value: Vec::new(),
            string_array_value: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParameterValue {}
