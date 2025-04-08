use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionPlanRequest {
    pub workspace_parameters: crate::moveit_msgs::msg::WorkspaceParameters,
    pub start_state: crate::moveit_msgs::msg::RobotState,
    pub goal_constraints: Vec<crate::moveit_msgs::msg::Constraints>,
    pub path_constraints: crate::moveit_msgs::msg::Constraints,
    pub trajectory_constraints: crate::moveit_msgs::msg::TrajectoryConstraints,
    pub reference_trajectories: Vec<crate::moveit_msgs::msg::GenericTrajectory>,
    pub pipeline_id: ::std::string::String,
    pub planner_id: ::std::string::String,
    pub group_name: ::std::string::String,
    pub num_planning_attempts: i32,
    pub allowed_planning_time: f64,
    pub max_velocity_scaling_factor: f64,
    pub max_acceleration_scaling_factor: f64,
    pub cartesian_speed_limited_link: ::std::string::String,
    pub max_cartesian_speed: f64,
}

impl Default for MotionPlanRequest {
    fn default() -> Self {
        MotionPlanRequest {
            workspace_parameters: crate::moveit_msgs::msg::WorkspaceParameters::default(),
            start_state: crate::moveit_msgs::msg::RobotState::default(),
            goal_constraints: Vec::new(),
            path_constraints: crate::moveit_msgs::msg::Constraints::default(),
            trajectory_constraints: crate::moveit_msgs::msg::TrajectoryConstraints::default(),
            reference_trajectories: Vec::new(),
            pipeline_id: ::std::string::String::new(),
            planner_id: ::std::string::String::new(),
            group_name: ::std::string::String::new(),
            num_planning_attempts: 0,
            allowed_planning_time: 0.0,
            max_velocity_scaling_factor: 0.0,
            max_acceleration_scaling_factor: 0.0,
            cartesian_speed_limited_link: ::std::string::String::new(),
            max_cartesian_speed: 0.0,
        }
    }
}

impl ros2_client::Message for MotionPlanRequest {}
