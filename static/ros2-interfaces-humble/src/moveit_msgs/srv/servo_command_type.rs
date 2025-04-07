use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoCommandTypeRequest {
    pub command_type: i8,
}

impl ServoCommandTypeRequest {
    pub const JOINT_JOG: i8 = 0;
    pub const TWIST: i8 = 1;
    pub const POSE: i8 = 2;
}

impl Default for ServoCommandTypeRequest {
    fn default() -> Self {
        ServoCommandTypeRequest {
            command_type: 0,
        }
    }
}

impl ros2_client::Message for ServoCommandTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoCommandTypeResponse {
    pub success: bool,
}

impl Default for ServoCommandTypeResponse {
    fn default() -> Self {
        ServoCommandTypeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ServoCommandTypeResponse {}


pub struct ServoCommandType;
impl ros2_client::Service for ServoCommandType {
    type Request = ServoCommandTypeRequest;
    type Response = ServoCommandTypeResponse;

    fn request_type_name(&self) -> &str { "ServoCommandTypeRequest" }
    fn response_type_name(&self) -> &str { "ServoCommandTypeResponse" }
}
