use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraRecordVideoRequest {
    pub payload_index: u8, // default: 1
    pub start_stop: bool,
}

impl Default for CameraRecordVideoRequest {
    fn default() -> Self {
        CameraRecordVideoRequest {
            payload_index: 1,
            start_stop: false,
        }
    }
}

impl ros2_client::Message for CameraRecordVideoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraRecordVideoResponse {
    pub success: bool,
}

impl Default for CameraRecordVideoResponse {
    fn default() -> Self {
        CameraRecordVideoResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraRecordVideoResponse {}


pub struct CameraRecordVideo;
impl ros2_client::Service for CameraRecordVideo {
    type Request = CameraRecordVideoRequest;
    type Response = CameraRecordVideoResponse;

    fn request_type_name(&self) -> &str { "CameraRecordVideoRequest" }
    fn response_type_name(&self) -> &str { "CameraRecordVideoResponse" }
}
