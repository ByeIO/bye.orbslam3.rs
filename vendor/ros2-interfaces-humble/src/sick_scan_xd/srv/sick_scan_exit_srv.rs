use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvRequest {

}

impl Default for SickScanExitSrvRequest {
    fn default() -> Self {
        SickScanExitSrvRequest {

        }
    }
}

impl ros2_client::Message for SickScanExitSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvResponse {
    pub success: bool,
}

impl Default for SickScanExitSrvResponse {
    fn default() -> Self {
        SickScanExitSrvResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SickScanExitSrvResponse {}


pub struct SickScanExitSrv;
impl ros2_client::Service for SickScanExitSrv {
    type Request = SickScanExitSrvRequest;
    type Response = SickScanExitSrvResponse;

    fn request_type_name(&self) -> &str { "SickScanExitSrvRequest" }
    fn response_type_name(&self) -> &str { "SickScanExitSrvResponse" }
}
