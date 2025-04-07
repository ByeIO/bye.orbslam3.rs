use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContaminationResultSrvRequest {

}

impl Default for GetContaminationResultSrvRequest {
    fn default() -> Self {
        GetContaminationResultSrvRequest {

        }
    }
}

impl ros2_client::Message for GetContaminationResultSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContaminationResultSrvResponse {
    pub warning: u8,
    pub error: u8,
    pub success: bool,
}

impl Default for GetContaminationResultSrvResponse {
    fn default() -> Self {
        GetContaminationResultSrvResponse {
            warning: 0,
            error: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for GetContaminationResultSrvResponse {}


pub struct GetContaminationResultSrv;
impl ros2_client::Service for GetContaminationResultSrv {
    type Request = GetContaminationResultSrvRequest;
    type Response = GetContaminationResultSrvResponse;

    fn request_type_name(&self) -> &str { "GetContaminationResultSrvRequest" }
    fn response_type_name(&self) -> &str { "GetContaminationResultSrvResponse" }
}
