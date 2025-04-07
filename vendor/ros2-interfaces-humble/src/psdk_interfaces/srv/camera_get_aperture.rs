use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetApertureRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetApertureRequest {
    fn default() -> Self {
        CameraGetApertureRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetApertureRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetApertureResponse {
    pub success: bool,
    pub aperture: u16,
}

impl Default for CameraGetApertureResponse {
    fn default() -> Self {
        CameraGetApertureResponse {
            success: false,
            aperture: 0,
        }
    }
}

impl ros2_client::Message for CameraGetApertureResponse {}


pub struct CameraGetAperture;
impl ros2_client::Service for CameraGetAperture {
    type Request = CameraGetApertureRequest;
    type Response = CameraGetApertureResponse;

    fn request_type_name(&self) -> &str { "CameraGetApertureRequest" }
    fn response_type_name(&self) -> &str { "CameraGetApertureResponse" }
}
