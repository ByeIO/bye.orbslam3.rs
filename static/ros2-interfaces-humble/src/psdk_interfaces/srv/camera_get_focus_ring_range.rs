use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusRingRangeRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetFocusRingRangeRequest {
    fn default() -> Self {
        CameraGetFocusRingRangeRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetFocusRingRangeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFocusRingRangeResponse {
    pub success: bool,
    pub min_value: i32,
    pub max_value: i32,
}

impl Default for CameraGetFocusRingRangeResponse {
    fn default() -> Self {
        CameraGetFocusRingRangeResponse {
            success: false,
            min_value: 0,
            max_value: 0,
        }
    }
}

impl ros2_client::Message for CameraGetFocusRingRangeResponse {}


pub struct CameraGetFocusRingRange;
impl ros2_client::Service for CameraGetFocusRingRange {
    type Request = CameraGetFocusRingRangeRequest;
    type Response = CameraGetFocusRingRangeResponse;

    fn request_type_name(&self) -> &str { "CameraGetFocusRingRangeRequest" }
    fn response_type_name(&self) -> &str { "CameraGetFocusRingRangeResponse" }
}
