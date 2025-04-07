use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainConstantsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for GetDomainConstantsRequest {
    fn default() -> Self {
        GetDomainConstantsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainConstantsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainConstantsResponse {
    pub success: bool,
    pub constants: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainConstantsResponse {
    fn default() -> Self {
        GetDomainConstantsResponse {
            success: false,
            constants: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainConstantsResponse {}


pub struct GetDomainConstants;
impl ros2_client::Service for GetDomainConstants {
    type Request = GetDomainConstantsRequest;
    type Response = GetDomainConstantsResponse;

    fn request_type_name(&self) -> &str { "GetDomainConstantsRequest" }
    fn response_type_name(&self) -> &str { "GetDomainConstantsResponse" }
}
