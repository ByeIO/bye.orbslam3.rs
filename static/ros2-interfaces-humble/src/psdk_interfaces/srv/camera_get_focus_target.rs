use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusTargetRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetFocusTargetRequest {
    fn default() -> Self {
        CameraGetFocusTargetRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetFocusTargetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusTargetResponse {
    pub success: bool,
    pub x_target: f32,
    pub y_target: f32,
}

impl Default for CameraGetFocusTargetResponse {
    fn default() -> Self {
        CameraGetFocusTargetResponse {
            success: false,
            x_target: 0.0,
            y_target: 0.0,
        }
    }
}

impl ros2_client::Message for CameraGetFocusTargetResponse {}


pub struct CameraGetFocusTarget;
impl ros2_client::Service for CameraGetFocusTarget {
    type Request = CameraGetFocusTargetRequest;
    type Response = CameraGetFocusTargetResponse;

    fn request_type_name(&self) -> &str { "CameraGetFocusTargetRequest" }
    fn response_type_name(&self) -> &str { "CameraGetFocusTargetResponse" }
}
