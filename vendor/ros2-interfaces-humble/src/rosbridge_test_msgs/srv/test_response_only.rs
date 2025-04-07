use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyRequest {

}

impl Default for TestResponseOnlyRequest {
    fn default() -> Self {
        TestResponseOnlyRequest {

        }
    }
}

impl ros2_client::Message for TestResponseOnlyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyResponse {
    pub data: i32,
}

impl Default for TestResponseOnlyResponse {
    fn default() -> Self {
        TestResponseOnlyResponse {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestResponseOnlyResponse {}


pub struct TestResponseOnly;
impl ros2_client::Service for TestResponseOnly {
    type Request = TestResponseOnlyRequest;
    type Response = TestResponseOnlyResponse;

    fn request_type_name(&self) -> &str { "TestResponseOnlyRequest" }
    fn response_type_name(&self) -> &str { "TestResponseOnlyResponse" }
}
