use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalSetModeRequest {
    pub payload_index: u8, // default: 1
    pub gimbal_mode: u8,
}

impl Default for GimbalSetModeRequest {
    fn default() -> Self {
        GimbalSetModeRequest {
            payload_index: 1,
            gimbal_mode: 0,
        }
    }
}

impl ros2_client::Message for GimbalSetModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalSetModeResponse {
    pub success: bool,
}

impl Default for GimbalSetModeResponse {
    fn default() -> Self {
        GimbalSetModeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for GimbalSetModeResponse {}


pub struct GimbalSetMode;
impl ros2_client::Service for GimbalSetMode {
    type Request = GimbalSetModeRequest;
    type Response = GimbalSetModeResponse;

    fn request_type_name(&self) -> &str { "GimbalSetModeRequest" }
    fn response_type_name(&self) -> &str { "GimbalSetModeResponse" }
}
