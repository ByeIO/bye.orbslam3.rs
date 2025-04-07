use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyRequest {
    pub data: i32,
}

impl Default for TestRequestOnlyRequest {
    fn default() -> Self {
        TestRequestOnlyRequest {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestOnlyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyResponse {

}

impl Default for TestRequestOnlyResponse {
    fn default() -> Self {
        TestRequestOnlyResponse {

        }
    }
}

impl ros2_client::Message for TestRequestOnlyResponse {}


pub struct TestRequestOnly;
impl ros2_client::Service for TestRequestOnly {
    type Request = TestRequestOnlyRequest;
    type Response = TestRequestOnlyResponse;

    fn request_type_name(&self) -> &str { "TestRequestOnlyRequest" }
    fn response_type_name(&self) -> &str { "TestRequestOnlyResponse" }
}
