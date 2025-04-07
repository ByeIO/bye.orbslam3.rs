use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityPoseRequest {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetEntityPoseRequest {
    fn default() -> Self {
        SetEntityPoseRequest {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SetEntityPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityPoseResponse {
    pub success: bool,
}

impl Default for SetEntityPoseResponse {
    fn default() -> Self {
        SetEntityPoseResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetEntityPoseResponse {}


pub struct SetEntityPose;
impl ros2_client::Service for SetEntityPose {
    type Request = SetEntityPoseRequest;
    type Response = SetEntityPoseResponse;

    fn request_type_name(&self) -> &str { "SetEntityPoseRequest" }
    fn response_type_name(&self) -> &str { "SetEntityPoseResponse" }
}
