use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    pub key: ::std::string::String,
    pub value: Vec<::std::string::String>,
}

impl Default for Trait {
    fn default() -> Self {
        Trait {
            key: ::std::string::String::new(),
            value: Vec::new(),
        }
    }
}

impl ros2_client::Message for Trait {}
