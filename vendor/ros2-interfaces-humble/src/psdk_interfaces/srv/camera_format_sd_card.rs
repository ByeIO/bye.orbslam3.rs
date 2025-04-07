use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraFormatSdCardRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraFormatSdCardRequest {
    fn default() -> Self {
        CameraFormatSdCardRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraFormatSdCardRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraFormatSdCardResponse {
    pub success: bool,
}

impl Default for CameraFormatSdCardResponse {
    fn default() -> Self {
        CameraFormatSdCardResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraFormatSdCardResponse {}


pub struct CameraFormatSdCard;
impl ros2_client::Service for CameraFormatSdCard {
    type Request = CameraFormatSdCardRequest;
    type Response = CameraFormatSdCardResponse;

    fn request_type_name(&self) -> &str { "CameraFormatSdCardRequest" }
    fn response_type_name(&self) -> &str { "CameraFormatSdCardResponse" }
}
