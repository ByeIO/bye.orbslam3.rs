use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DummyRequest {

}

impl Default for DummyRequest {
    fn default() -> Self {
        DummyRequest {

        }
    }
}

impl ros2_client::Message for DummyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DummyResponse {

}

impl Default for DummyResponse {
    fn default() -> Self {
        DummyResponse {

        }
    }
}

impl ros2_client::Message for DummyResponse {}


pub struct Dummy;
impl ros2_client::Service for Dummy {
    type Request = DummyRequest;
    type Response = DummyResponse;

    fn request_type_name(&self) -> &str { "DummyRequest" }
    fn response_type_name(&self) -> &str { "DummyResponse" }
}
