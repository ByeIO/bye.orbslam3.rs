use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub mode_request_id: u64,
    #[serde(rename = "type")]    pub type_: u32,
    pub at_checkpoint: u32,
}

impl PauseRequest {
    pub const TYPE_PAUSE_IMMEDIATELY: u32 = 0;
    pub const TYPE_PAUSE_AT_CHECKPOINT: u32 = 1;
    pub const TYPE_RESUME: u32 = 2;
}

impl Default for PauseRequest {
    fn default() -> Self {
        PauseRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            mode_request_id: 0,
            type_: 0,
            at_checkpoint: 0,
        }
    }
}

impl ros2_client::Message for PauseRequest {}
