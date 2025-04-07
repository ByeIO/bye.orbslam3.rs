use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadSplitEvent {
    pub closed_file: ::std::string::String,
    pub opened_file: ::std::string::String,
}

impl Default for ReadSplitEvent {
    fn default() -> Self {
        ReadSplitEvent {
            closed_file: ::std::string::String::new(),
            opened_file: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReadSplitEvent {}
