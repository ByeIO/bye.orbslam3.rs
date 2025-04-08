use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvRequest {

}

impl Default for SCsoftresetSrvRequest {
    fn default() -> Self {
        SCsoftresetSrvRequest {

        }
    }
}

impl ros2_client::Message for SCsoftresetSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvResponse {
    pub success: bool,
}

impl Default for SCsoftresetSrvResponse {
    fn default() -> Self {
        SCsoftresetSrvResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SCsoftresetSrvResponse {}


pub struct SCsoftresetSrv;
impl ros2_client::Service for SCsoftresetSrv {
    type Request = SCsoftresetSrvRequest;
    type Response = SCsoftresetSrvResponse;

    fn request_type_name(&self) -> &str { "SCsoftresetSrvRequest" }
    fn response_type_name(&self) -> &str { "SCsoftresetSrvResponse" }
}
