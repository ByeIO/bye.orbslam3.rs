use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraspPlanningRequest {
    pub group_name: ::std::string::String,
    pub target: crate::moveit_msgs::msg::CollisionObject,
    pub support_surfaces: Vec<::std::string::String>,
    pub candidate_grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub movable_obstacles: Vec<crate::moveit_msgs::msg::CollisionObject>,
}

impl Default for GraspPlanningRequest {
    fn default() -> Self {
        GraspPlanningRequest {
            group_name: ::std::string::String::new(),
            target: crate::moveit_msgs::msg::CollisionObject::default(),
            support_surfaces: Vec::new(),
            candidate_grasps: Vec::new(),
            movable_obstacles: Vec::new(),
        }
    }
}

impl ros2_client::Message for GraspPlanningRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraspPlanningResponse {
    pub grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GraspPlanningResponse {
    fn default() -> Self {
        GraspPlanningResponse {
            grasps: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for GraspPlanningResponse {}


pub struct GraspPlanning;
impl ros2_client::Service for GraspPlanning {
    type Request = GraspPlanningRequest;
    type Response = GraspPlanningResponse;

    fn request_type_name(&self) -> &str { "GraspPlanningRequest" }
    fn response_type_name(&self) -> &str { "GraspPlanningResponse" }
}
