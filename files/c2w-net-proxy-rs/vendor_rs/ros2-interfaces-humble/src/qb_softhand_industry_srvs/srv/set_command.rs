use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandRequest {
    pub max_repeats: i32,
    pub set_commands: bool,
    pub position_command: i16,
}

impl Default for SetCommandRequest {
    fn default() -> Self {
        SetCommandRequest {
            max_repeats: 0,
            set_commands: false,
            position_command: 0,
        }
    }
}

impl ros2_client::Message for SetCommandRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandResponse {
    pub success: bool,
}

impl Default for SetCommandResponse {
    fn default() -> Self {
        SetCommandResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetCommandResponse {}


pub struct SetCommand;
impl ros2_client::Service for SetCommand {
    type Request = SetCommandRequest;
    type Response = SetCommandResponse;

    fn request_type_name(&self) -> &str { "SetCommandRequest" }
    fn response_type_name(&self) -> &str { "SetCommandResponse" }
}
