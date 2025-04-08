use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToRequest {
    pub target: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for MakePlanToRequest {
    fn default() -> Self {
        MakePlanToRequest {
            target: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

impl ros2_client::Message for MakePlanToRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToResponse {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanToResponse {
    fn default() -> Self {
        MakePlanToResponse {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

impl ros2_client::Message for MakePlanToResponse {}


pub struct MakePlanTo;
impl ros2_client::Service for MakePlanTo {
    type Request = MakePlanToRequest;
    type Response = MakePlanToResponse;

    fn request_type_name(&self) -> &str { "MakePlanToRequest" }
    fn response_type_name(&self) -> &str { "MakePlanToResponse" }
}
