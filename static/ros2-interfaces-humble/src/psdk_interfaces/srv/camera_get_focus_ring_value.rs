use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusRingValueRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetFocusRingValueRequest {
    fn default() -> Self {
        CameraGetFocusRingValueRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetFocusRingValueRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusRingValueResponse {
    pub success: bool,
    pub focus_ring_value: i32,
}

impl Default for CameraGetFocusRingValueResponse {
    fn default() -> Self {
        CameraGetFocusRingValueResponse {
            success: false,
            focus_ring_value: 0,
        }
    }
}

impl ros2_client::Message for CameraGetFocusRingValueResponse {}


pub struct CameraGetFocusRingValue;
impl ros2_client::Service for CameraGetFocusRingValue {
    type Request = CameraGetFocusRingValueRequest;
    type Response = CameraGetFocusRingValueResponse;

    fn request_type_name(&self) -> &str { "CameraGetFocusRingValueRequest" }
    fn response_type_name(&self) -> &str { "CameraGetFocusRingValueResponse" }
}
