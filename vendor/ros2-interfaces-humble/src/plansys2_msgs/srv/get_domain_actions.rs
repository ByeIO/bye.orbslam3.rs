use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionsRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainActionsRequest {
    fn default() -> Self {
        GetDomainActionsRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainActionsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionsResponse {
    pub success: bool,
    pub actions: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionsResponse {
    fn default() -> Self {
        GetDomainActionsResponse {
            success: false,
            actions: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionsResponse {}


pub struct GetDomainActions;
impl ros2_client::Service for GetDomainActions {
    type Request = GetDomainActionsRequest;
    type Response = GetDomainActionsResponse;

    fn request_type_name(&self) -> &str { "GetDomainActionsRequest" }
    fn response_type_name(&self) -> &str { "GetDomainActionsResponse" }
}
