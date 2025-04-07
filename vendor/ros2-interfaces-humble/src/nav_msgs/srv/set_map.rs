use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapRequest {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetMapRequest {
    fn default() -> Self {
        SetMapRequest {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapResponse {
    pub success: bool,
}

impl Default for SetMapResponse {
    fn default() -> Self {
        SetMapResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetMapResponse {}


pub struct SetMap;
impl ros2_client::Service for SetMap {
    type Request = SetMapRequest;
    type Response = SetMapResponse;

    fn request_type_name(&self) -> &str { "SetMapRequest" }
    fn response_type_name(&self) -> &str { "SetMapResponse" }
}
