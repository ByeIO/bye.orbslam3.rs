use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetISORequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetISORequest {
    fn default() -> Self {
        CameraGetISORequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetISORequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetISOResponse {
    pub success: bool,
    pub iso_factor: i8,
}

impl Default for CameraGetISOResponse {
    fn default() -> Self {
        CameraGetISOResponse {
            success: false,
            iso_factor: 0,
        }
    }
}

impl ros2_client::Message for CameraGetISOResponse {}


pub struct CameraGetISO;
impl ros2_client::Service for CameraGetISO {
    type Request = CameraGetISORequest;
    type Response = CameraGetISOResponse;

    fn request_type_name(&self) -> &str { "CameraGetISORequest" }
    fn response_type_name(&self) -> &str { "CameraGetISOResponse" }
}
