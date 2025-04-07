use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetExposureModeEVRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetExposureModeEVRequest {
    fn default() -> Self {
        CameraGetExposureModeEVRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetExposureModeEVRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetExposureModeEVResponse {
    pub success: bool,
    pub exposure_mode: u8,
    pub ev_factor: u8,
}

impl Default for CameraGetExposureModeEVResponse {
    fn default() -> Self {
        CameraGetExposureModeEVResponse {
            success: false,
            exposure_mode: 0,
            ev_factor: 0,
        }
    }
}

impl ros2_client::Message for CameraGetExposureModeEVResponse {}


pub struct CameraGetExposureModeEV;
impl ros2_client::Service for CameraGetExposureModeEV {
    type Request = CameraGetExposureModeEVRequest;
    type Response = CameraGetExposureModeEVResponse;

    fn request_type_name(&self) -> &str { "CameraGetExposureModeEVRequest" }
    fn response_type_name(&self) -> &str { "CameraGetExposureModeEVResponse" }
}
