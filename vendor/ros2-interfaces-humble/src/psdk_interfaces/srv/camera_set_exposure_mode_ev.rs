use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetExposureModeEVRequest {
    pub payload_index: u8, // default: 1
    pub exposure_mode: u8,
    pub ev_factor: u8, // default: 255
}

impl Default for CameraSetExposureModeEVRequest {
    fn default() -> Self {
        CameraSetExposureModeEVRequest {
            payload_index: 1,
            exposure_mode: 0,
            ev_factor: 255,
        }
    }
}

impl ros2_client::Message for CameraSetExposureModeEVRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetExposureModeEVResponse {
    pub success: bool,
}

impl Default for CameraSetExposureModeEVResponse {
    fn default() -> Self {
        CameraSetExposureModeEVResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetExposureModeEVResponse {}


pub struct CameraSetExposureModeEV;
impl ros2_client::Service for CameraSetExposureModeEV {
    type Request = CameraSetExposureModeEVRequest;
    type Response = CameraSetExposureModeEVResponse;

    fn request_type_name(&self) -> &str { "CameraSetExposureModeEVRequest" }
    fn response_type_name(&self) -> &str { "CameraSetExposureModeEVResponse" }
}
