use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetISORequest {
    pub payload_index: u8, // default: 1
    pub iso_factor: u8,
}

impl Default for CameraSetISORequest {
    fn default() -> Self {
        CameraSetISORequest {
            payload_index: 1,
            iso_factor: 0,
        }
    }
}

impl ros2_client::Message for CameraSetISORequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetISOResponse {
    pub success: bool,
}

impl Default for CameraSetISOResponse {
    fn default() -> Self {
        CameraSetISOResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetISOResponse {}


pub struct CameraSetISO;
impl ros2_client::Service for CameraSetISO {
    type Request = CameraSetISORequest;
    type Response = CameraSetISOResponse;

    fn request_type_name(&self) -> &str { "CameraSetISORequest" }
    fn response_type_name(&self) -> &str { "CameraSetISOResponse" }
}
