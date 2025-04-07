use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub sub_types: Vec<::std::string::String>,
}

impl Default for Param {
    fn default() -> Self {
        Param {
            name: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            sub_types: Vec::new(),
        }
    }
}

impl ros2_client::Message for Param {}
