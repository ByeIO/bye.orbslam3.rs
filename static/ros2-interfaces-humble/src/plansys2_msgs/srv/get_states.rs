use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStatesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetStatesRequest {
    fn default() -> Self {
        GetStatesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStatesResponse {
    pub success: bool,
    pub states: Vec<crate::plansys2_msgs::msg::Node>,
    pub error_info: ::std::string::String,
}

impl Default for GetStatesResponse {
    fn default() -> Self {
        GetStatesResponse {
            success: false,
            states: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetStatesResponse {}


pub struct GetStates;
impl ros2_client::Service for GetStates {
    type Request = GetStatesRequest;
    type Response = GetStatesResponse;

    fn request_type_name(&self) -> &str { "GetStatesRequest" }
    fn response_type_name(&self) -> &str { "GetStatesResponse" }
}
