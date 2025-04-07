use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDurativeActionDetailsRequest {
    pub durative_action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainDurativeActionDetailsRequest {
    fn default() -> Self {
        GetDomainDurativeActionDetailsRequest {
            durative_action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDurativeActionDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDurativeActionDetailsResponse {
    pub success: bool,
    pub durative_action: crate::plansys2_msgs::msg::DurativeAction,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDurativeActionDetailsResponse {
    fn default() -> Self {
        GetDomainDurativeActionDetailsResponse {
            success: false,
            durative_action: crate::plansys2_msgs::msg::DurativeAction::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDurativeActionDetailsResponse {}


pub struct GetDomainDurativeActionDetails;
impl ros2_client::Service for GetDomainDurativeActionDetails {
    type Request = GetDomainDurativeActionDetailsRequest;
    type Response = GetDomainDurativeActionDetailsResponse;

    fn request_type_name(&self) -> &str { "GetDomainDurativeActionDetailsRequest" }
    fn response_type_name(&self) -> &str { "GetDomainDurativeActionDetailsResponse" }
}
