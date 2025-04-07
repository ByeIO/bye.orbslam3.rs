use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandLongRequest {
    pub broadcast: bool,
    pub command: u16,
    pub confirmation: u8,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub param5: f32,
    pub param6: f32,
    pub param7: f32,
}

impl Default for CommandLongRequest {
    fn default() -> Self {
        CommandLongRequest {
            broadcast: false,
            command: 0,
            confirmation: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
        }
    }
}

impl ros2_client::Message for CommandLongRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandLongResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandLongResponse {
    fn default() -> Self {
        CommandLongResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandLongResponse {}


pub struct CommandLong;
impl ros2_client::Service for CommandLong {
    type Request = CommandLongRequest;
    type Response = CommandLongResponse;

    fn request_type_name(&self) -> &str { "CommandLongRequest" }
    fn response_type_name(&self) -> &str { "CommandLongResponse" }
}
