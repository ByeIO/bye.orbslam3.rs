use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetShutterSpeedRequest {
    pub payload_index: u8, // default: 1
    pub shutter_speed_factor: u8,
}

impl Default for CameraSetShutterSpeedRequest {
    fn default() -> Self {
        CameraSetShutterSpeedRequest {
            payload_index: 1,
            shutter_speed_factor: 0,
        }
    }
}

impl ros2_client::Message for CameraSetShutterSpeedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetShutterSpeedResponse {
    pub success: bool,
}

impl Default for CameraSetShutterSpeedResponse {
    fn default() -> Self {
        CameraSetShutterSpeedResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CameraSetShutterSpeedResponse {}


pub struct CameraSetShutterSpeed;
impl ros2_client::Service for CameraSetShutterSpeed {
    type Request = CameraSetShutterSpeedRequest;
    type Response = CameraSetShutterSpeedResponse;

    fn request_type_name(&self) -> &str { "CameraSetShutterSpeedRequest" }
    fn response_type_name(&self) -> &str { "CameraSetShutterSpeedResponse" }
}
