use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstanceDetailsRequest {
    pub instance: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsRequest {
    fn default() -> Self {
        GetProblemInstanceDetailsRequest {
            instance: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstanceDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstanceDetailsResponse {
    pub success: bool,
    pub instance: crate::plansys2_msgs::msg::Param,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsResponse {
    fn default() -> Self {
        GetProblemInstanceDetailsResponse {
            success: false,
            instance: crate::plansys2_msgs::msg::Param::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstanceDetailsResponse {}


pub struct GetProblemInstanceDetails;
impl ros2_client::Service for GetProblemInstanceDetails {
    type Request = GetProblemInstanceDetailsRequest;
    type Response = GetProblemInstanceDetailsResponse;

    fn request_type_name(&self) -> &str { "GetProblemInstanceDetailsRequest" }
    fn response_type_name(&self) -> &str { "GetProblemInstanceDetailsResponse" }
}
