use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootSinglePhotoRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraShootSinglePhotoRequest {
    fn default() -> Self {
        CameraShootSinglePhotoRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraShootSinglePhotoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootSinglePhotoResponse {
    pub success: bool,
}

impl Default for CameraShootSinglePhotoResponse {
    fn default() -> Self {
        CameraShootSinglePhotoResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraShootSinglePhotoResponse {}


pub struct CameraShootSinglePhoto;
impl ros2_client::Service for CameraShootSinglePhoto {
    type Request = CameraShootSinglePhotoRequest;
    type Response = CameraShootSinglePhotoResponse;

    fn request_type_name(&self) -> &str { "CameraShootSinglePhotoRequest" }
    fn response_type_name(&self) -> &str { "CameraShootSinglePhotoResponse" }
}
