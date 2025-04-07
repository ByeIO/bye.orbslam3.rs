use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLRequest {
    pub min_pitch: f32,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandTOLRequest {
    fn default() -> Self {
        CommandTOLRequest {
            min_pitch: 0.0,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

impl ros2_client::Message for CommandTOLRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLResponse {
    fn default() -> Self {
        CommandTOLResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTOLResponse {}


pub struct CommandTOL;
impl ros2_client::Service for CommandTOL {
    type Request = CommandTOLRequest;
    type Response = CommandTOLResponse;

    fn request_type_name(&self) -> &str { "CommandTOLRequest" }
    fn response_type_name(&self) -> &str { "CommandTOLResponse" }
}
