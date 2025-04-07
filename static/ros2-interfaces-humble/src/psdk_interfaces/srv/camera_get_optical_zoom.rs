use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetOpticalZoomRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetOpticalZoomRequest {
    fn default() -> Self {
        CameraGetOpticalZoomRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetOpticalZoomRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetOpticalZoomResponse {
    pub success: bool,
    pub zoom_factor: f32,
    pub max_zoom_factor: f32,
}

impl Default for CameraGetOpticalZoomResponse {
    fn default() -> Self {
        CameraGetOpticalZoomResponse {
            success: false,
            zoom_factor: 0.0,
            max_zoom_factor: 0.0,
        }
    }
}

impl ros2_client::Message for CameraGetOpticalZoomResponse {}


pub struct CameraGetOpticalZoom;
impl ros2_client::Service for CameraGetOpticalZoom {
    type Request = CameraGetOpticalZoomRequest;
    type Response = CameraGetOpticalZoomResponse;

    fn request_type_name(&self) -> &str { "CameraGetOpticalZoomRequest" }
    fn response_type_name(&self) -> &str { "CameraGetOpticalZoomResponse" }
}
