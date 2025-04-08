use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandFeedback {
    pub command: ::std::string::String,
    pub args: Vec<::std::string::String>,
}

impl Default for CommandFeedback {
    fn default() -> Self {
        CommandFeedback {
            command: ::std::string::String::new(),
            args: Vec::new(),
        }
    }
}

impl ros2_client::Message for CommandFeedback {}
