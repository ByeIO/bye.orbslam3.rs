use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetLaserRangingInfoRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetLaserRangingInfoRequest {
    fn default() -> Self {
        CameraGetLaserRangingInfoRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetLaserRangingInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetLaserRangingInfoResponse {
    pub success: bool,
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: i32,
    pub distance: i32,
    pub screen_x: i16,
    pub screen_y: i16,
    pub enable_lidar: bool,
    pub exception: u8,
}

impl Default for CameraGetLaserRangingInfoResponse {
    fn default() -> Self {
        CameraGetLaserRangingInfoResponse {
            success: false,
            longitude: 0.0,
            latitude: 0.0,
            altitude: 0,
            distance: 0,
            screen_x: 0,
            screen_y: 0,
            enable_lidar: false,
            exception: 0,
        }
    }
}

impl ros2_client::Message for CameraGetLaserRangingInfoResponse {}


pub struct CameraGetLaserRangingInfo;
impl ros2_client::Service for CameraGetLaserRangingInfo {
    type Request = CameraGetLaserRangingInfoRequest;
    type Response = CameraGetLaserRangingInfoResponse;

    fn request_type_name(&self) -> &str { "CameraGetLaserRangingInfoRequest" }
    fn response_type_name(&self) -> &str { "CameraGetLaserRangingInfoResponse" }
}
