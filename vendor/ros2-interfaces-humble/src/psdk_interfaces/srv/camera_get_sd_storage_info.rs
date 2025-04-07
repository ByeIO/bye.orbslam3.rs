use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetSDStorageInfoRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetSDStorageInfoRequest {
    fn default() -> Self {
        CameraGetSDStorageInfoRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetSDStorageInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetSDStorageInfoResponse {
    pub success: bool,
    pub total_capacity: u32,
    pub remain_capacity: u32,
}

impl Default for CameraGetSDStorageInfoResponse {
    fn default() -> Self {
        CameraGetSDStorageInfoResponse {
            success: false,
            total_capacity: 0,
            remain_capacity: 0,
        }
    }
}

impl ros2_client::Message for CameraGetSDStorageInfoResponse {}


pub struct CameraGetSDStorageInfo;
impl ros2_client::Service for CameraGetSDStorageInfo {
    type Request = CameraGetSDStorageInfoRequest;
    type Response = CameraGetSDStorageInfoResponse;

    fn request_type_name(&self) -> &str { "CameraGetSDStorageInfoRequest" }
    fn response_type_name(&self) -> &str { "CameraGetSDStorageInfoResponse" }
}
