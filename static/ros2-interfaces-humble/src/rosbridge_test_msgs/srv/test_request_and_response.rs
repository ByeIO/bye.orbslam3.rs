use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseRequest {
    pub data: i32,
}

impl Default for TestRequestAndResponseRequest {
    fn default() -> Self {
        TestRequestAndResponseRequest {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestAndResponseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseResponse {
    pub data: i32,
}

impl Default for TestRequestAndResponseResponse {
    fn default() -> Self {
        TestRequestAndResponseResponse {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestAndResponseResponse {}


pub struct TestRequestAndResponse;
impl ros2_client::Service for TestRequestAndResponse {
    type Request = TestRequestAndResponseRequest;
    type Response = TestRequestAndResponseResponse;

    fn request_type_name(&self) -> &str { "TestRequestAndResponseRequest" }
    fn response_type_name(&self) -> &str { "TestRequestAndResponseResponse" }
}
