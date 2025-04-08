use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetApertureRequest {
    pub payload_index: u8, // default: 1
    pub aperture: u16,
}

impl Default for CameraSetApertureRequest {
    fn default() -> Self {
        CameraSetApertureRequest {
            payload_index: 1,
            aperture: 0,
        }
    }
}

impl ros2_client::Message for CameraSetApertureRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetApertureResponse {
    pub success: bool,
}

impl Default for CameraSetApertureResponse {
    fn default() -> Self {
        CameraSetApertureResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetApertureResponse {}


pub struct CameraSetAperture;
impl ros2_client::Service for CameraSetAperture {
    type Request = CameraSetApertureRequest;
    type Response = CameraSetApertureResponse;

    fn request_type_name(&self) -> &str { "CameraSetApertureRequest" }
    fn response_type_name(&self) -> &str { "CameraSetApertureResponse" }
}
