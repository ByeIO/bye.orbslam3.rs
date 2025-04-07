use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetInfraredZoomRequest {
    pub payload_index: u8, // default: 1
    pub zoom_factor: f32,
}

impl Default for CameraSetInfraredZoomRequest {
    fn default() -> Self {
        CameraSetInfraredZoomRequest {
            payload_index: 1,
            zoom_factor: 0.0,
        }
    }
}

impl ros2_client::Message for CameraSetInfraredZoomRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetInfraredZoomResponse {
    pub success: bool,
}

impl Default for CameraSetInfraredZoomResponse {
    fn default() -> Self {
        CameraSetInfraredZoomResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetInfraredZoomResponse {}


pub struct CameraSetInfraredZoom;
impl ros2_client::Service for CameraSetInfraredZoom {
    type Request = CameraSetInfraredZoomRequest;
    type Response = CameraSetInfraredZoomResponse;

    fn request_type_name(&self) -> &str { "CameraSetInfraredZoomRequest" }
    fn response_type_name(&self) -> &str { "CameraSetInfraredZoomResponse" }
}
