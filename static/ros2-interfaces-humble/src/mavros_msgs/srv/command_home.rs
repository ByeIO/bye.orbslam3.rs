use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandHomeRequest {
    pub current_gps: bool,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandHomeRequest {
    fn default() -> Self {
        CommandHomeRequest {
            current_gps: false,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

impl ros2_client::Message for CommandHomeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandHomeResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandHomeResponse {
    fn default() -> Self {
        CommandHomeResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandHomeResponse {}


pub struct CommandHome;
impl ros2_client::Service for CommandHome {
    type Request = CommandHomeRequest;
    type Response = CommandHomeResponse;

    fn request_type_name(&self) -> &str { "CommandHomeRequest" }
    fn response_type_name(&self) -> &str { "CommandHomeResponse" }
}
