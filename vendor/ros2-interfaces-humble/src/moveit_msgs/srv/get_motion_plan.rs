use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanRequest {
    pub motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest,
}

impl Default for GetMotionPlanRequest {
    fn default() -> Self {
        GetMotionPlanRequest {
            motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest::default(),
        }
    }
}

impl ros2_client::Message for GetMotionPlanRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanResponse {
    pub motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse,
}

impl Default for GetMotionPlanResponse {
    fn default() -> Self {
        GetMotionPlanResponse {
            motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse::default(),
        }
    }
}

impl ros2_client::Message for GetMotionPlanResponse {}


pub struct GetMotionPlan;
impl ros2_client::Service for GetMotionPlan {
    type Request = GetMotionPlanRequest;
    type Response = GetMotionPlanResponse;

    fn request_type_name(&self) -> &str { "GetMotionPlanRequest" }
    fn response_type_name(&self) -> &str { "GetMotionPlanResponse" }
}
