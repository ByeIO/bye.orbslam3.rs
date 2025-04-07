use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalResetRequest {
    pub payload_index: u8, // default: 1
    pub reset_mode: u8, // default: 1
}

impl Default for GimbalResetRequest {
    fn default() -> Self {
        GimbalResetRequest {
            payload_index: 1,
            reset_mode: 1,
        }
    }
}

impl ros2_client::Message for GimbalResetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalResetResponse {
    pub success: bool,
}

impl Default for GimbalResetResponse {
    fn default() -> Self {
        GimbalResetResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for GimbalResetResponse {}


pub struct GimbalReset;
impl ros2_client::Service for GimbalReset {
    type Request = GimbalResetRequest;
    type Response = GimbalResetResponse;

    fn request_type_name(&self) -> &str { "GimbalResetRequest" }
    fn response_type_name(&self) -> &str { "GimbalResetResponse" }
}
