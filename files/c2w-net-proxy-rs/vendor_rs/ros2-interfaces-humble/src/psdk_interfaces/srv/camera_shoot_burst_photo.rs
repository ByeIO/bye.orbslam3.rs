use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootBurstPhotoRequest {
    pub payload_index: u8, // default: 1
    pub photo_burst_count: u8,
}

impl Default for CameraShootBurstPhotoRequest {
    fn default() -> Self {
        CameraShootBurstPhotoRequest {
            payload_index: 1,
            photo_burst_count: 0,
        }
    }
}

impl ros2_client::Message for CameraShootBurstPhotoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootBurstPhotoResponse {
    pub success: bool,
}

impl Default for CameraShootBurstPhotoResponse {
    fn default() -> Self {
        CameraShootBurstPhotoResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraShootBurstPhotoResponse {}


pub struct CameraShootBurstPhoto;
impl ros2_client::Service for CameraShootBurstPhoto {
    type Request = CameraShootBurstPhotoRequest;
    type Response = CameraShootBurstPhotoResponse;

    fn request_type_name(&self) -> &str { "CameraShootBurstPhotoRequest" }
    fn response_type_name(&self) -> &str { "CameraShootBurstPhotoResponse" }
}
