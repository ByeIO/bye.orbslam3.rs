use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanRequest {
    pub domain: ::std::string::String,
    pub problem: ::std::string::String,
}

impl Default for GetPlanRequest {
    fn default() -> Self {
        GetPlanRequest {
            domain: ::std::string::String::new(),
            problem: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlanRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanResponse {
    pub success: bool,
    pub plan: crate::plansys2_msgs::msg::Plan,
    pub error_info: ::std::string::String,
}

impl Default for GetPlanResponse {
    fn default() -> Self {
        GetPlanResponse {
            success: false,
            plan: crate::plansys2_msgs::msg::Plan::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlanResponse {}


pub struct GetPlan;
impl ros2_client::Service for GetPlan {
    type Request = GetPlanRequest;
    type Response = GetPlanResponse;

    fn request_type_name(&self) -> &str { "GetPlanRequest" }
    fn response_type_name(&self) -> &str { "GetPlanResponse" }
}
