use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UICommand {
    pub command: ::std::string::String,
    pub key: ::std::string::String,
}

impl Default for UICommand {
    fn default() -> Self {
        UICommand {
            command: ::std::string::String::new(),
            key: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UICommand {}
