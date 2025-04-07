use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetShutterSpeedRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetShutterSpeedRequest {
    fn default() -> Self {
        CameraGetShutterSpeedRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetShutterSpeedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetShutterSpeedResponse {
    pub success: bool,
    pub shutter_speed: i8,
}

impl Default for CameraGetShutterSpeedResponse {
    fn default() -> Self {
        CameraGetShutterSpeedResponse {
            success: false,
            shutter_speed: 0,
        }
    }
}

impl ros2_client::Message for CameraGetShutterSpeedResponse {}


pub struct CameraGetShutterSpeed;
impl ros2_client::Service for CameraGetShutterSpeed {
    type Request = CameraGetShutterSpeedRequest;
    type Response = CameraGetShutterSpeedResponse;

    fn request_type_name(&self) -> &str { "CameraGetShutterSpeedRequest" }
    fn response_type_name(&self) -> &str { "CameraGetShutterSpeedResponse" }
}
