use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTrajectoryRequest {
    pub configuration_directory: ::std::string::String,
    pub configuration_basename: ::std::string::String,
    pub use_initial_pose: bool,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub relative_to_trajectory_id: i32,
}

impl Default for StartTrajectoryRequest {
    fn default() -> Self {
        StartTrajectoryRequest {
            configuration_directory: ::std::string::String::new(),
            configuration_basename: ::std::string::String::new(),
            use_initial_pose: false,
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            relative_to_trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for StartTrajectoryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTrajectoryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory_id: i32,
}

impl Default for StartTrajectoryResponse {
    fn default() -> Self {
        StartTrajectoryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for StartTrajectoryResponse {}


pub struct StartTrajectory;
impl ros2_client::Service for StartTrajectory {
    type Request = StartTrajectoryRequest;
    type Response = StartTrajectoryResponse;

    fn request_type_name(&self) -> &str { "StartTrajectoryRequest" }
    fn response_type_name(&self) -> &str { "StartTrajectoryResponse" }
}
