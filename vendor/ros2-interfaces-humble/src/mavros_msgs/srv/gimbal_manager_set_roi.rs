use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetRoiRequest {
    pub mode: u8,
    pub gimbal_device_id: u8,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub pitch_offset: f32,
    pub roll_offset: f32,
    pub yaw_offset: f32,
    pub sysid: u8,
}

impl GimbalManagerSetRoiRequest {
    pub const ROI_MODE_LOCATION: u8 = 0;
    pub const ROI_MODE_WP_NEXT_OFFSET: u8 = 1;
    pub const ROI_MODE_SYSID: u8 = 2;
    pub const ROI_MODE_NONE: u8 = 3;
}

impl Default for GimbalManagerSetRoiRequest {
    fn default() -> Self {
        GimbalManagerSetRoiRequest {
            mode: 0,
            gimbal_device_id: 0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            pitch_offset: 0.0,
            roll_offset: 0.0,
            yaw_offset: 0.0,
            sysid: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetRoiRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetRoiResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerSetRoiResponse {
    fn default() -> Self {
        GimbalManagerSetRoiResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetRoiResponse {}


pub struct GimbalManagerSetRoi;
impl ros2_client::Service for GimbalManagerSetRoi {
    type Request = GimbalManagerSetRoiRequest;
    type Response = GimbalManagerSetRoiResponse;

    fn request_type_name(&self) -> &str { "GimbalManagerSetRoiRequest" }
    fn response_type_name(&self) -> &str { "GimbalManagerSetRoiResponse" }
}
