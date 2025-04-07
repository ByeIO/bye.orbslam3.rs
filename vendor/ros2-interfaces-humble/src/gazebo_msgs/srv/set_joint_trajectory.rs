use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointTrajectoryRequest {
    pub model_name: ::std::string::String,
    pub joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory,
    pub model_pose: crate::geometry_msgs::msg::Pose,
    pub set_model_pose: bool,
    pub disable_physics_updates: bool,
}

impl Default for SetJointTrajectoryRequest {
    fn default() -> Self {
        SetJointTrajectoryRequest {
            model_name: ::std::string::String::new(),
            joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory::default(),
            model_pose: crate::geometry_msgs::msg::Pose::default(),
            set_model_pose: false,
            disable_physics_updates: false,
        }
    }
}

impl ros2_client::Message for SetJointTrajectoryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointTrajectoryResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointTrajectoryResponse {
    fn default() -> Self {
        SetJointTrajectoryResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetJointTrajectoryResponse {}


pub struct SetJointTrajectory;
impl ros2_client::Service for SetJointTrajectory {
    type Request = SetJointTrajectoryRequest;
    type Response = SetJointTrajectoryResponse;

    fn request_type_name(&self) -> &str { "SetJointTrajectoryRequest" }
    fn response_type_name(&self) -> &str { "SetJointTrajectoryResponse" }
}
