use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandsRequest {
    pub id: i32,
    pub max_repeats: i32,
    pub set_commands: bool,
    pub set_commands_async: bool,
    pub commands: Vec<i16>,
}

impl Default for SetCommandsRequest {
    fn default() -> Self {
        SetCommandsRequest {
            id: 0,
            max_repeats: 0,
            set_commands: false,
            set_commands_async: false,
            commands: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetCommandsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCommandsResponse {
    pub success: bool,
    pub failures: i32,
}

impl Default for SetCommandsResponse {
    fn default() -> Self {
        SetCommandsResponse {
            success: false,
            failures: 0,
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
