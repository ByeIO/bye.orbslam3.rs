use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandsRequest {
    pub max_repeats: i32,
    pub set_commands: bool,
    pub position_command: i16,
    pub velocity_command: i16,
    pub current_command: i16,
}

impl Default for SetCommandsRequest {
    fn default() -> Self {
        SetCommandsRequest {
            max_repeats: 0,
            set_commands: false,
            position_command: 0,
            velocity_command: 0,
            current_command: 0,
        }
    }
}

impl ros2_client::Message for SetCommandsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandsResponse {
    pub success: bool,
}

impl Default for SetCommandsResponse {
    fn default() -> Self {
        SetCommandsResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetCommandsResponse {}


pub struct SetCommands;
impl ros2_client::Service for SetCommands {
    type Request = SetCommandsRequest;
    type Response = SetCommandsResponse;

    fn request_type_name(&self) -> &str { "SetCommandsRequest" }
    fn response_type_name(&self) -> &str { "SetCommandsResponse" }
}
