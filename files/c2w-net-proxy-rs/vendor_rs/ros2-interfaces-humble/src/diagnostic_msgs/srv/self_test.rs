use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfTestRequest {

}

impl Default for SelfTestRequest {
    fn default() -> Self {
        SelfTestRequest {

        }
    }
}

impl ros2_client::Message for SelfTestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfTestResponse {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl Default for SelfTestResponse {
    fn default() -> Self {
        SelfTestResponse {
            id: ::std::string::String::new(),
            passed: 0,
            status: Vec::new(),
        }
    }
}

impl ros2_client::Message for SelfTestResponse {}


pub struct SelfTest;
impl ros2_client::Service for SelfTest {
    type Request = SelfTestRequest;
    type Response = SelfTestResponse;

    fn request_type_name(&self) -> &str { "SelfTestRequest" }
    fn response_type_name(&self) -> &str { "SelfTestResponse" }
}
