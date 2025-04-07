use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsRequest {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleRequestFieldsRequest {
    fn default() -> Self {
        TestMultipleRequestFieldsRequest {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

impl ros2_client::Message for TestMultipleRequestFieldsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsResponse {

}

impl Default for TestMultipleRequestFieldsResponse {
    fn default() -> Self {
        TestMultipleRequestFieldsResponse {

        }
    }
}

impl ros2_client::Message for TestMultipleRequestFieldsResponse {}


pub struct TestMultipleRequestFields;
impl ros2_client::Service for TestMultipleRequestFields {
    type Request = TestMultipleRequestFieldsRequest;
    type Response = TestMultipleRequestFieldsResponse;

    fn request_type_name(&self) -> &str { "TestMultipleRequestFieldsRequest" }
    fn response_type_name(&self) -> &str { "TestMultipleRequestFieldsResponse" }
}
