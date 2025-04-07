use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeRequest {

}

impl Default for GetSafetyModeRequest {
    fn default() -> Self {
        GetSafetyModeRequest {

        }
    }
}

impl ros2_client::Message for GetSafetyModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeResponse {
    pub safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetSafetyModeResponse {
    fn default() -> Self {
        GetSafetyModeResponse {
            safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetSafetyModeResponse {}


pub struct GetSafetyMode;
impl ros2_client::Service for GetSafetyMode {
    type Request = GetSafetyModeRequest;
    type Response = GetSafetyModeResponse;

    fn request_type_name(&self) -> &str { "GetSafetyModeRequest" }
    fn response_type_name(&self) -> &str { "GetSafetyModeResponse" }
}
