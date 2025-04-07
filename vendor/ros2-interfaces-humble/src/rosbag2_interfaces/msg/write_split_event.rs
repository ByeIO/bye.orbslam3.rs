use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteSplitEvent {
    pub closed_file: ::std::string::String,
    pub opened_file: ::std::string::String,
}

impl Default for WriteSplitEvent {
    fn default() -> Self {
        WriteSplitEvent {
            closed_file: ::std::string::String::new(),
            opened_file: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for WriteSplitEvent {}
