use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestRequest {
    pub int_values: Vec<i32>,
}

impl Default for TestArrayRequestRequest {
    fn default() -> Self {
        TestArrayRequestRequest {
            int_values: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestArrayRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestResponse {

}

impl Default for TestArrayRequestResponse {
    fn default() -> Self {
        TestArrayRequestResponse {

        }
    }
}

impl ros2_client::Message for TestArrayRequestResponse {}


pub struct TestArrayRequest;
impl ros2_client::Service for TestArrayRequest {
    type Request = TestArrayRequestRequest;
    type Response = TestArrayRequestResponse;

    fn request_type_name(&self) -> &str { "TestArrayRequestRequest" }
    fn response_type_name(&self) -> &str { "TestArrayRequestResponse" }
}
