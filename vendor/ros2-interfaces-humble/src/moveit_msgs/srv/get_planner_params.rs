use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlannerParamsRequest {
    pub pipeline_id: ::std::string::String,
    pub planner_config: ::std::string::String,
    pub group: ::std::string::String,
}

impl Default for GetPlannerParamsRequest {
    fn default() -> Self {
        GetPlannerParamsRequest {
            pipeline_id: ::std::string::String::new(),
            planner_config: ::std::string::String::new(),
            group: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlannerParamsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlannerParamsResponse {
    pub params: crate::moveit_msgs::msg::PlannerParams,
}

impl Default for GetPlannerParamsResponse {
    fn default() -> Self {
        GetPlannerParamsResponse {
            params: crate::moveit_msgs::msg::PlannerParams::default(),
        }
    }
}

impl ros2_client::Message for GetPlannerParamsResponse {}


pub struct GetPlannerParams;
impl ros2_client::Service for GetPlannerParams {
    type Request = GetPlannerParamsRequest;
    type Response = GetPlannerParamsResponse;

    fn request_type_name(&self) -> &str { "GetPlannerParamsRequest" }
    fn response_type_name(&self) -> &str { "GetPlannerParamsResponse" }
}
