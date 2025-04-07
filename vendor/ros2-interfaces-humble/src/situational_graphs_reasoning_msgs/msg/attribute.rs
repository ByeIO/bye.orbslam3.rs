use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub name: ::std::string::String,
    pub str_value: ::std::string::String,
    pub fl_value: Vec<f64>,
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute {
            name: ::std::string::String::new(),
            str_value: ::std::string::String::new(),
            fl_value: Vec::new(),
        }
    }
}

impl ros2_client::Message for Attribute {}
