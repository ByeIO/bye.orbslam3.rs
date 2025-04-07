use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetOpticalZoomRequest {
    pub payload_index: u8, // default: 1
    pub zoom_factor: f32,
}

impl Default for CameraSetOpticalZoomRequest {
    fn default() -> Self {
        CameraSetOpticalZoomRequest {
            payload_index: 1,
            zoom_factor: 0.0,
        }
    }
}

impl ros2_client::Message for CameraSetOpticalZoomRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetOpticalZoomResponse {
    pub success: bool,
}

impl Default for CameraSetOpticalZoomResponse {
    fn default() -> Self {
        CameraSetOpticalZoomResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetOpticalZoomResponse {}


pub struct CameraSetOpticalZoom;
impl ros2_client::Service for CameraSetOpticalZoom {
    type Request = CameraSetOpticalZoomRequest;
    type Response = CameraSetOpticalZoomResponse;

    fn request_type_name(&self) -> &str { "CameraSetOpticalZoomRequest" }
    fn response_type_name(&self) -> &str { "CameraSetOpticalZoomResponse" }
}
