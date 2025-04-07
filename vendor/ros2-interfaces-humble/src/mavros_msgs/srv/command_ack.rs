use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAckRequest {
    pub command: u16,
    pub result: u8,
    pub progress: u8,
    pub result_param2: u32,
}

impl Default for CommandAckRequest {
    fn default() -> Self {
        CommandAckRequest {
            command: 0,
            result: 0,
            progress: 0,
            result_param2: 0,
        }
    }
}

impl ros2_client::Message for CommandAckRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAckResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandAckResponse {
    fn default() -> Self {
        CommandAckResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandAckResponse {}


pub struct CommandAck;
impl ros2_client::Service for CommandAck {
    type Request = CommandAckRequest;
    type Response = CommandAckResponse;

    fn request_type_name(&self) -> &str { "CommandAckRequest" }
    fn response_type_name(&self) -> &str { "CommandAckResponse" }
}
