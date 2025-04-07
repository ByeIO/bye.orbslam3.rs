use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainRequest {
    fn default() -> Self {
        GetDomainRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainResponse {
    pub success: bool,
    pub domain: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainResponse {
    fn default() -> Self {
        GetDomainResponse {
            success: false,
            domain: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainResponse {}


pub struct GetDomain;
impl ros2_client::Service for GetDomain {
    type Request = GetDomainRequest;
    type Response = GetDomainResponse;

    fn request_type_name(&self) -> &str { "GetDomainRequest" }
    fn response_type_name(&self) -> &str { "GetDomainResponse" }
}
