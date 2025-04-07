use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraStopShootPhotoRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraStopShootPhotoRequest {
    fn default() -> Self {
        CameraStopShootPhotoRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraStopShootPhotoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraStopShootPhotoResponse {
    pub success: bool,
}

impl Default for CameraStopShootPhotoResponse {
    fn default() -> Self {
        CameraStopShootPhotoResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraStopShootPhotoResponse {}


pub struct CameraStopShootPhoto;
impl ros2_client::Service for CameraStopShootPhoto {
    type Request = CameraStopShootPhotoRequest;
    type Response = CameraStopShootPhotoResponse;

    fn request_type_name(&self) -> &str { "CameraStopShootPhotoRequest" }
    fn response_type_name(&self) -> &str { "CameraStopShootPhotoResponse" }
}
