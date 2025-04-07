use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub outcome: ::std::string::String,
    pub state: ::std::string::String,
}

impl Default for Transition {
    fn default() -> Self {
        Transition {
            outcome: ::std::string::String::new(),
            state: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Transition {}
