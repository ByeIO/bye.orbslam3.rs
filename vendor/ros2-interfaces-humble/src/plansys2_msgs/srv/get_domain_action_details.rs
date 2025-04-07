use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionDetailsRequest {
    pub action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainActionDetailsRequest {
    fn default() -> Self {
        GetDomainActionDetailsRequest {
            action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionDetailsResponse {
    pub action: crate::plansys2_msgs::msg::Action,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionDetailsResponse {
    fn default() -> Self {
        GetDomainActionDetailsResponse {
            action: crate::plansys2_msgs::msg::Action::default(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionDetailsResponse {}


pub struct GetDomainActionDetails;
impl ros2_client::Service for GetDomainActionDetails {
    type Request = GetDomainActionDetailsRequest;
    type Response = GetDomainActionDetailsResponse;

    fn request_type_name(&self) -> &str { "GetDomainActionDetailsRequest" }
    fn response_type_name(&self) -> &str { "GetDomainActionDetailsResponse" }
}
