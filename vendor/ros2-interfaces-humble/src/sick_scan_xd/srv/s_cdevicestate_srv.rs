use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvRequest {

}

impl Default for SCdevicestateSrvRequest {
    fn default() -> Self {
        SCdevicestateSrvRequest {

        }
    }
}

impl ros2_client::Message for SCdevicestateSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvResponse {
    pub state: i32,
    pub success: bool,
}

impl Default for SCdevicestateSrvResponse {
    fn default() -> Self {
        SCdevicestateSrvResponse {
            state: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for SCdevicestateSrvResponse {}


pub struct SCdevicestateSrv;
impl ros2_client::Service for SCdevicestateSrv {
    type Request = SCdevicestateSrvRequest;
    type Response = SCdevicestateSrvResponse;

    fn request_type_name(&self) -> &str { "SCdevicestateSrvRequest" }
    fn response_type_name(&self) -> &str { "SCdevicestateSrvResponse" }
}
