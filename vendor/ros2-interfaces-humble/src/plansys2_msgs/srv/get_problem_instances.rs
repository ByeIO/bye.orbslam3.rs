use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstancesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemInstancesRequest {
    fn default() -> Self {
        GetProblemInstancesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemInstancesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstancesResponse {
    pub success: bool,
    pub instances: Vec<crate::plansys2_msgs::msg::Param>,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstancesResponse {
    fn default() -> Self {
        GetProblemInstancesResponse {
            success: false,
            instances: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstancesResponse {}


pub struct GetProblemInstances;
impl ros2_client::Service for GetProblemInstances {
    type Request = GetProblemInstancesRequest;
    type Response = GetProblemInstancesResponse;

    fn request_type_name(&self) -> &str { "GetProblemInstancesRequest" }
    fn response_type_name(&self) -> &str { "GetProblemInstancesResponse" }
}
