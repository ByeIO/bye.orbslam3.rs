use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    pub key: ::std::string::String,
    pub name: ::std::string::String,
    pub usage: ::std::string::String,
    pub description: ::std::string::String,
    pub joints: Vec<::std::string::String>,
    pub positions: Vec<f64>,
    pub times_from_start: Vec<f64>,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            key: ::std::string::String::new(),
            name: ::std::string::String::new(),
            usage: ::std::string::String::new(),
            description: ::std::string::String::new(),
            joints: Vec::new(),
            positions: Vec::new(),
            times_from_start: Vec::new(),
        }
    }
}

impl ros2_client::Message for Motion {}
