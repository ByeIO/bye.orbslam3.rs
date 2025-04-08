use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvRequest {
    pub active: bool,
}

impl Default for ECRChangeArrSrvRequest {
    fn default() -> Self {
        ECRChangeArrSrvRequest {
            active: false,
        }
    }
}

impl ros2_client::Message for ECRChangeArrSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvResponse {
    pub success: bool,
}

impl Default for ECRChangeArrSrvResponse {
    fn default() -> Self {
        ECRChangeArrSrvResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ECRChangeArrSrvResponse {}


pub struct ECRChangeArrSrv;
impl ros2_client::Service for ECRChangeArrSrv {
    type Request = ECRChangeArrSrvRequest;
    type Response = ECRChangeArrSrvResponse;

    fn request_type_name(&self) -> &str { "ECRChangeArrSrvRequest" }
    fn response_type_name(&self) -> &str { "ECRChangeArrSrvResponse" }
}
