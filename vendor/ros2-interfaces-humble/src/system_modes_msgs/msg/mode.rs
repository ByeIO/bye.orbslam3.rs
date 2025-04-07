use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mode {
    pub label: ::std::string::String,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Mode {}
