use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateRequest {
    pub template_id: ::std::string::String,
}

impl Default for WarmupTemplateRequest {
    fn default() -> Self {
        WarmupTemplateRequest {
            template_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for WarmupTemplateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for WarmupTemplateResponse {
    fn default() -> Self {
        WarmupTemplateResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for WarmupTemplateResponse {}


pub struct WarmupTemplate;
impl ros2_client::Service for WarmupTemplate {
    type Request = WarmupTemplateRequest;
    type Response = WarmupTemplateResponse;

    fn request_type_name(&self) -> &str { "WarmupTemplateRequest" }
    fn response_type_name(&self) -> &str { "WarmupTemplateResponse" }
}
