use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODORequest {

}

impl Default for ResetODORequest {
    fn default() -> Self {
        ResetODORequest {

        }
    }
}

impl ros2_client::Message for ResetODORequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODOResponse {

}

impl Default for ResetODOResponse {
    fn default() -> Self {
        ResetODOResponse {

        }
    }
}

impl ros2_client::Message for ResetODOResponse {}


pub struct ResetODO;
impl ros2_client::Service for ResetODO {
    type Request = ResetODORequest;
    type Response = ResetODOResponse;

    fn request_type_name(&self) -> &str { "ResetODORequest" }
    fn response_type_name(&self) -> &str { "ResetODOResponse" }
}
