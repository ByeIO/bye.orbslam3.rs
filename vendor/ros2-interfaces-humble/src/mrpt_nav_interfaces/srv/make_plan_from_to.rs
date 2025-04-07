use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanFromToRequest {
    pub start: crate::geometry_msgs::msg::Pose,
    pub target: crate::geometry_msgs::msg::Pose,
}

impl Default for MakePlanFromToRequest {
    fn default() -> Self {
        MakePlanFromToRequest {
            start: crate::geometry_msgs::msg::Pose::default(),
            target: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for MakePlanFromToRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanFromToResponse {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanFromToResponse {
    fn default() -> Self {
        MakePlanFromToResponse {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

impl ros2_client::Message for MakePlanFromToResponse {}


pub struct MakePlanFromTo;
impl ros2_client::Service for MakePlanFromTo {
    type Request = MakePlanFromToRequest;
    type Response = MakePlanFromToResponse;

    fn request_type_name(&self) -> &str { "MakePlanFromToRequest" }
    fn response_type_name(&self) -> &str { "MakePlanFromToResponse" }
}
