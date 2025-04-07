use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColdStartRequest {
    pub reset_type: u8,
}

impl ColdStartRequest {
    pub const HW_RESET_IMMEDIATELY: u8 = 0x00;
    pub const SW_RESET_CONTROLLED: u8 = 0x01;
    pub const SW_RESET_CONTROLLED_GNSS: u8 = 0x02;
    pub const HW_RESET_AFTER_SHUTDOWN: u8 = 0x04;
    pub const GNSS_STOP_CONTROLLED: u8 = 0x08;
    pub const GNSS_START_CONTROLLED: u8 = 0x09;
}

impl Default for ColdStartRequest {
    fn default() -> Self {
        ColdStartRequest {
            reset_type: 0,
        }
    }
}

impl ros2_client::Message for ColdStartRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColdStartResponse {

}

impl Default for ColdStartResponse {
    fn default() -> Self {
        ColdStartResponse {

        }
    }
}

impl ros2_client::Message for ColdStartResponse {}


pub struct ColdStart;
impl ros2_client::Service for ColdStart {
    type Request = ColdStartRequest;
    type Response = ColdStartResponse;

    fn request_type_name(&self) -> &str { "ColdStartRequest" }
    fn response_type_name(&self) -> &str { "ColdStartResponse" }
}
