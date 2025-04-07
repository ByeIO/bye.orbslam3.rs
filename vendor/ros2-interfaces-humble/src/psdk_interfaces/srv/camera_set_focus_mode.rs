use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusModeRequest {
    pub payload_index: u8, // default: 1
    pub focus_mode: u8,
}

impl Default for CameraSetFocusModeRequest {
    fn default() -> Self {
        CameraSetFocusModeRequest {
            payload_index: 1,
            focus_mode: 0,
        }
    }
}

impl ros2_client::Message for CameraSetFocusModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusModeResponse {
    pub success: bool,
}

impl Default for CameraSetFocusModeResponse {
    fn default() -> Self {
        CameraSetFocusModeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetFocusModeResponse {}


pub struct CameraSetFocusMode;
impl ros2_client::Service for CameraSetFocusMode {
    type Request = CameraSetFocusModeRequest;
    type Response = CameraSetFocusModeResponse;

    fn request_type_name(&self) -> &str { "CameraSetFocusModeRequest" }
    fn response_type_name(&self) -> &str { "CameraSetFocusModeResponse" }
}
