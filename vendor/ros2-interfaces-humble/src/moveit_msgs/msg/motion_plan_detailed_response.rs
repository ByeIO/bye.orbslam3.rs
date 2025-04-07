use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionPlanDetailedResponse {
    pub trajectory_start: crate::moveit_msgs::msg::RobotState,
    pub group_name: ::std::string::String,
    pub trajectory: Vec<crate::moveit_msgs::msg::RobotTrajectory>,
    pub description: Vec<::std::string::String>,
    pub processing_time: Vec<f64>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for MotionPlanDetailedResponse {
    fn default() -> Self {
        MotionPlanDetailedResponse {
            trajectory_start: crate::moveit_msgs::msg::RobotState::default(),
            group_name: ::std::string::String::new(),
            trajectory: Vec::new(),
            description: Vec::new(),
            processing_time: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for MotionPlanDetailedResponse {}
