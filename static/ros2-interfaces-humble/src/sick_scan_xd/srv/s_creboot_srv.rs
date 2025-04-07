use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCrebootSrvRequest {

}

impl Default for SCrebootSrvRequest {
    fn default() -> Self {
        SCrebootSrvRequest {

        }
    }
}

impl ros2_client::Message for SCrebootSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCrebootSrvResponse {
    pub success: bool,
}

impl Default for SCrebootSrvResponse {
    fn default() -> Self {
        SCrebootSrvResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SCrebootSrvResponse {}


pub struct SCrebootSrv;
impl ros2_client::Service for SCrebootSrv {
    type Request = SCrebootSrvRequest;
    type Response = SCrebootSrvResponse;

    fn request_type_name(&self) -> &str { "SCrebootSrvRequest" }
    fn response_type_name(&self) -> &str { "SCrebootSrvResponse" }
}
