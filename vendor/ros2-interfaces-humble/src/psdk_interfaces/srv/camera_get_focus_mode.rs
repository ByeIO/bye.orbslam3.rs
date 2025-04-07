use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusModeRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetFocusModeRequest {
    fn default() -> Self {
        CameraGetFocusModeRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetFocusModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusModeResponse {
    pub success: bool,
    pub focus_mode: u8,
}

impl Default for CameraGetFocusModeResponse {
    fn default() -> Self {
        CameraGetFocusModeResponse {
            success: false,
            focus_mode: 0,
        }
    }
}

impl ros2_client::Message for CameraGetFocusModeResponse {}


pub struct CameraGetFocusMode;
impl ros2_client::Service for CameraGetFocusMode {
    type Request = CameraGetFocusModeRequest;
    type Response = CameraGetFocusModeResponse;

    fn request_type_name(&self) -> &str { "CameraGetFocusModeRequest" }
    fn response_type_name(&self) -> &str { "CameraGetFocusModeResponse" }
}
