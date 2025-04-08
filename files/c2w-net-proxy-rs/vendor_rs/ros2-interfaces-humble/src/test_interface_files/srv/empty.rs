use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyRequest {

}

impl Default for EmptyRequest {
    fn default() -> Self {
        EmptyRequest {

        }
    }
}

impl ros2_client::Message for EmptyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyResponse {

}

impl Default for EmptyResponse {
    fn default() -> Self {
        EmptyResponse {

        }
    }
}

impl ros2_client::Message for EmptyResponse {}


pub struct Empty;
impl ros2_client::Service for Empty {
    type Request = EmptyRequest;
    type Response = EmptyResponse;

    fn request_type_name(&self) -> &str { "EmptyRequest" }
    fn response_type_name(&self) -> &str { "EmptyResponse" }
}
