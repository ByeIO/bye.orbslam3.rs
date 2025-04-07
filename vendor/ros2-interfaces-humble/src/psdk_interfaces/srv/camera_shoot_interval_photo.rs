use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootIntervalPhotoRequest {
    pub payload_index: u8, // default: 1
    pub num_photos_to_capture: u8,
    pub time_interval: i16,
}

impl Default for CameraShootIntervalPhotoRequest {
    fn default() -> Self {
        CameraShootIntervalPhotoRequest {
            payload_index: 1,
            num_photos_to_capture: 0,
            time_interval: 0,
        }
    }
}

impl ros2_client::Message for CameraShootIntervalPhotoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraShootIntervalPhotoResponse {
    pub success: bool,
}

impl Default for CameraShootIntervalPhotoResponse {
    fn default() -> Self {
        CameraShootIntervalPhotoResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraShootIntervalPhotoResponse {}


pub struct CameraShootIntervalPhoto;
impl ros2_client::Service for CameraShootIntervalPhoto {
    type Request = CameraShootIntervalPhotoRequest;
    type Response = CameraShootIntervalPhotoResponse;

    fn request_type_name(&self) -> &str { "CameraShootIntervalPhotoRequest" }
    fn response_type_name(&self) -> &str { "CameraShootIntervalPhotoResponse" }
}
