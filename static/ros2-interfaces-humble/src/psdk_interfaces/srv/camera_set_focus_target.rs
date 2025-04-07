use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusTargetRequest {
    pub payload_index: u8, // default: 1
    pub x_target: f32,
    pub y_target: f32,
}

impl Default for CameraSetFocusTargetRequest {
    fn default() -> Self {
        CameraSetFocusTargetRequest {
            payload_index: 1,
            x_target: 0.0,
            y_target: 0.0,
        }
    }
}

impl ros2_client::Message for CameraSetFocusTargetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusTargetResponse {
    pub success: bool,
}

impl Default for CameraSetFocusTargetResponse {
    fn default() -> Self {
        CameraSetFocusTargetResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetFocusTargetResponse {}


pub struct CameraSetFocusTarget;
impl ros2_client::Service for CameraSetFocusTarget {
    type Request = CameraSetFocusTargetRequest;
    type Response = CameraSetFocusTargetResponse;

    fn request_type_name(&self) -> &str { "CameraSetFocusTargetRequest" }
    fn response_type_name(&self) -> &str { "CameraSetFocusTargetResponse" }
}
