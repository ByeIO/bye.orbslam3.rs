use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearRequest {

}

impl Default for ClearRequest {
    fn default() -> Self {
        ClearRequest {

        }
    }
}

impl ros2_client::Message for ClearRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearResponse {

}

impl Default for ClearResponse {
    fn default() -> Self {
        ClearResponse {

        }
    }
}

impl ros2_client::Message for ClearResponse {}


pub struct Clear;
impl ros2_client::Service for Clear {
    type Request = ClearRequest;
    type Response = ClearResponse;

    fn request_type_name(&self) -> &str { "ClearRequest" }
    fn response_type_name(&self) -> &str { "ClearResponse" }
}
