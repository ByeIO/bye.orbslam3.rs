use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSSelfTestRequest {

}

impl Default for MicroROSSelfTestRequest {
    fn default() -> Self {
        MicroROSSelfTestRequest {

        }
    }
}

impl ros2_client::Message for MicroROSSelfTestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSSelfTestResponse {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus,
}

impl Default for MicroROSSelfTestResponse {
    fn default() -> Self {
        MicroROSSelfTestResponse {
            id: ::std::string::String::new(),
            passed: 0,
            status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus::default(),
        }
    }
}

impl ros2_client::Message for MicroROSSelfTestResponse {}


pub struct MicroROSSelfTest;
impl ros2_client::Service for MicroROSSelfTest {
    type Request = MicroROSSelfTestRequest;
    type Response = MicroROSSelfTestResponse;

    fn request_type_name(&self) -> &str { "MicroROSSelfTestRequest" }
    fn response_type_name(&self) -> &str { "MicroROSSelfTestResponse" }
}
