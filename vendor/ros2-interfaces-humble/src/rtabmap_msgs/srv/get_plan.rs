use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanRequest {
    pub goal_node: i32,
    pub goal: crate::geometry_msgs::msg::PoseStamped,
    pub tolerance: f32,
}

impl Default for GetPlanRequest {
    fn default() -> Self {
        GetPlanRequest {
            goal_node: 0,
            goal: crate::geometry_msgs::msg::PoseStamped::default(),
            tolerance: 0.0,
        }
    }
}

impl ros2_client::Message for GetPlanRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanResponse {
    pub plan: crate::rtabmap_msgs::msg::Path,
}

impl Default for GetPlanResponse {
    fn default() -> Self {
        GetPlanResponse {
            plan: crate::rtabmap_msgs::msg::Path::default(),
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
