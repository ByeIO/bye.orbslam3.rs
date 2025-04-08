use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for ObjectProperty {
    fn default() -> Self {
        ObjectProperty {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ObjectProperty {}
