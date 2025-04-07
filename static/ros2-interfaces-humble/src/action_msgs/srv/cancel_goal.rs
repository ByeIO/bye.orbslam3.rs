use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelGoalRequest {
    pub goal_info: crate::action_msgs::msg::GoalInfo,
}

impl Default for CancelGoalRequest {
    fn default() -> Self {
        CancelGoalRequest {
            goal_info: crate::action_msgs::msg::GoalInfo::default(),
        }
    }
}

impl ros2_client::Message for CancelGoalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelGoalResponse {
    pub return_code: i8,
    pub goals_canceling: Vec<crate::action_msgs::msg::GoalInfo>,
}

impl CancelGoalResponse {
    pub const ERROR_NONE: i8 = 0;
    pub const ERROR_REJECTED: i8 = 1;
    pub const ERROR_UNKNOWN_GOAL_ID: i8 = 2;
    pub const ERROR_GOAL_TERMINATED: i8 = 3;
}

impl Default for CancelGoalResponse {
    fn default() -> Self {
        CancelGoalResponse {
            return_code: 0,
            goals_canceling: Vec::new(),
        }
    }
}

impl ros2_client::Message for CancelGoalResponse {}


pub struct CancelGoal;
impl ros2_client::Service for CancelGoal {
    type Request = CancelGoalRequest;
    type Response = CancelGoalResponse;

    fn request_type_name(&self) -> &str { "CancelGoalRequest" }
    fn response_type_name(&self) -> &str { "CancelGoalResponse" }
}
