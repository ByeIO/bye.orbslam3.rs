use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramState {
    pub state: ::std::string::String,
}

impl ProgramState {
    pub const STOPPED: &'static str = "STOPPED";
    pub const PLAYING: &'static str = "PLAYING";
    pub const PAUSED: &'static str = "PAUSED";
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            state: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ProgramState {}
