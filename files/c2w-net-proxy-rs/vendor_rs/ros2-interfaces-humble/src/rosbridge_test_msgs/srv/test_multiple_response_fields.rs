use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleResponseFieldsRequest {

}

impl Default for TestMultipleResponseFieldsRequest {
    fn default() -> Self {
        TestMultipleResponseFieldsRequest {

        }
    }
}

impl ros2_client::Message for TestMultipleResponseFieldsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleResponseFieldsResponse {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleResponseFieldsResponse {
    fn default() -> Self {
        TestMultipleResponseFieldsResponse {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

impl ros2_client::Message for TestMultipleResponseFieldsResponse {}


pub struct TestMultipleResponseFields;
impl ros2_client::Service for TestMultipleResponseFields {
    type Request = TestMultipleResponseFieldsRequest;
    type Response = TestMultipleResponseFieldsResponse;

    fn request_type_name(&self) -> &str { "TestMultipleResponseFieldsRequest" }
    fn response_type_name(&self) -> &str { "TestMultipleResponseFieldsResponse" }
}
