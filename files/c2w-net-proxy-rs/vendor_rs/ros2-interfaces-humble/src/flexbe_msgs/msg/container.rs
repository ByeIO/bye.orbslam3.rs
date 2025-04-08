use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub path: ::std::string::String,
    pub children: Vec<::std::string::String>,
    pub outcomes: Vec<::std::string::String>,
    pub transitions: Vec<::std::string::String>,
    pub autonomy: Vec<i8>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            path: ::std::string::String::new(),
            children: Vec::new(),
            outcomes: Vec::new(),
            transitions: Vec::new(),
            autonomy: Vec::new(),
        }
    }
}

impl ros2_client::Message for Container {}
