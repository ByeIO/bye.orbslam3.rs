use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetupStreamingRequest {
    pub payload_index: u8, // default: 1
    pub camera_source: u8, // default: 0
    pub start_stop: bool,
    pub decoded_output: bool, // default: 1
}

impl Default for CameraSetupStreamingRequest {
    fn default() -> Self {
        CameraSetupStreamingRequest {
            payload_index: 1,
            camera_source: 0,
            start_stop: false,
            decoded_output: true,
        }
    }
}

impl ros2_client::Message for CameraSetupStreamingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetupStreamingResponse {
    pub success: bool,
}

impl Default for CameraSetupStreamingResponse {
    fn default() -> Self {
        CameraSetupStreamingResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetupStreamingResponse {}


pub struct CameraSetupStreaming;
impl ros2_client::Service for CameraSetupStreaming {
    type Request = CameraSetupStreamingRequest;
    type Response = CameraSetupStreamingResponse;

    fn request_type_name(&self) -> &str { "CameraSetupStreamingRequest" }
    fn response_type_name(&self) -> &str { "CameraSetupStreamingResponse" }
}
