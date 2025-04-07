use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlModeCommandRequest {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub mode: u8,
}

impl ControlModeCommandRequest {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const AUTONOMOUS_STEER_ONLY: u8 = 2;
    pub const AUTONOMOUS_VELOCITY_ONLY: u8 = 3;
    pub const MANUAL: u8 = 4;
}

impl Default for ControlModeCommandRequest {
    fn default() -> Self {
        ControlModeCommandRequest {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            mode: 0,
        }
    }
}

impl ros2_client::Message for ControlModeCommandRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlModeCommandResponse {
    pub success: bool,
}

impl Default for ControlModeCommandResponse {
    fn default() -> Self {
        ControlModeCommandResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ControlModeCommandResponse {}


pub struct ControlModeCommand;
impl ros2_client::Service for ControlModeCommand {
    type Request = ControlModeCommandRequest;
    type Response = ControlModeCommandResponse;

    fn request_type_name(&self) -> &str { "ControlModeCommandRequest" }
    fn response_type_name(&self) -> &str { "ControlModeCommandResponse" }
}
