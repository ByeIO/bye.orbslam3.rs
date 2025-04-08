use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetInitialPoseRequest {
    fn default() -> Self {
        SetInitialPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetInitialPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseResponse {

}

impl Default for SetInitialPoseResponse {
    fn default() -> Self {
        SetInitialPoseResponse {

        }
    }
}

impl ros2_client::Message for SetInitialPoseResponse {}


pub struct SetInitialPose;
impl ros2_client::Service for SetInitialPose {
    type Request = SetInitialPoseRequest;
    type Response = SetInitialPoseResponse;

    fn request_type_name(&self) -> &str { "SetInitialPoseRequest" }
    fn response_type_name(&self) -> &str { "SetInitialPoseResponse" }
}
