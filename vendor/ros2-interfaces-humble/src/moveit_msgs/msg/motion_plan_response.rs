use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionPlanResponse {
    pub trajectory_start: crate::moveit_msgs::msg::RobotState,
    pub group_name: ::std::string::String,
    pub trajectory: crate::moveit_msgs::msg::RobotTrajectory,
    pub planning_time: f64,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for MotionPlanResponse {
    fn default() -> Self {
        MotionPlanResponse {
            trajectory_start: crate::moveit_msgs::msg::RobotState::default(),
            group_name: ::std::string::String::new(),
            trajectory: crate::moveit_msgs::msg::RobotTrajectory::default(),
            planning_time: 0.0,
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for MotionPlanResponse {}
