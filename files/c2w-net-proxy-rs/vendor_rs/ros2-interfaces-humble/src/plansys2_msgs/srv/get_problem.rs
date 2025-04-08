use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemRequest {
    fn default() -> Self {
        GetProblemRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemResponse {
    pub success: bool,
    pub problem: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemResponse {
    fn default() -> Self {
        GetProblemResponse {
            success: false,
            problem: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemResponse {}


pub struct GetProblem;
impl ros2_client::Service for GetProblem {
    type Request = GetProblemRequest;
    type Response = GetProblemResponse;

    fn request_type_name(&self) -> &str { "GetProblemRequest" }
    fn response_type_name(&self) -> &str { "GetProblemResponse" }
}
