use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCartesianPathRequest {
    pub header: crate::std_msgs::msg::Header,
    pub start_state: crate::moveit_msgs::msg::RobotState,
    pub group_name: ::std::string::String,
    pub link_name: ::std::string::String,
    pub waypoints: Vec<crate::geometry_msgs::msg::Pose>,
    pub max_step: f64,
    pub jump_threshold: f64,
    pub prismatic_jump_threshold: f64,
    pub revolute_jump_threshold: f64,
    pub avoid_collisions: bool,
    pub path_constraints: crate::moveit_msgs::msg::Constraints,
    pub max_velocity_scaling_factor: f64,
    pub max_acceleration_scaling_factor: f64,
    pub cartesian_speed_limited_link: ::std::string::String,
    pub max_cartesian_speed: f64,
}

impl Default for GetCartesianPathRequest {
    fn default() -> Self {
        GetCartesianPathRequest {
            header: crate::std_msgs::msg::Header::default(),
            start_state: crate::moveit_msgs::msg::RobotState::default(),
            group_name: ::std::string::String::new(),
            link_name: ::std::string::String::new(),
            waypoints: Vec::new(),
            max_step: 0.0,
            jump_threshold: 0.0,
            prismatic_jump_threshold: 0.0,
            revolute_jump_threshold: 0.0,
            avoid_collisions: false,
            path_constraints: crate::moveit_msgs::msg::Constraints::default(),
            max_velocity_scaling_factor: 0.0,
            max_acceleration_scaling_factor: 0.0,
            cartesian_speed_limited_link: ::std::string::String::new(),
            max_cartesian_speed: 0.0,
        }
    }
}

impl ros2_client::Message for GetCartesianPathRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCartesianPathResponse {
    pub start_state: crate::moveit_msgs::msg::RobotState,
    pub solution: crate::moveit_msgs::msg::RobotTrajectory,
    pub fraction: f64,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetCartesianPathResponse {
    fn default() -> Self {
        GetCartesianPathResponse {
            start_state: crate::moveit_msgs::msg::RobotState::default(),
            solution: crate::moveit_msgs::msg::RobotTrajectory::default(),
            fraction: 0.0,
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for GetCartesianPathResponse {}


pub struct GetCartesianPath;
impl ros2_client::Service for GetCartesianPath {
    type Request = GetCartesianPathRequest;
    type Response = GetCartesianPathResponse;

    fn request_type_name(&self) -> &str { "GetCartesianPathRequest" }
    fn response_type_name(&self) -> &str { "GetCartesianPathResponse" }
}
