use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPushRequest {
    pub start_index: u16,
    pub waypoints: Vec<crate::mavros_msgs::msg::Waypoint>,
}

impl Default for WaypointPushRequest {
    fn default() -> Self {
        WaypointPushRequest {
            start_index: 0,
            waypoints: Vec::new(),
        }
    }
}

impl ros2_client::Message for WaypointPushRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPushResponse {
    pub success: bool,
    pub wp_transfered: u32,
}

impl Default for WaypointPushResponse {
    fn default() -> Self {
        WaypointPushResponse {
            success: false,
            wp_transfered: 0,
        }
    }
}

impl ros2_client::Message for WaypointPushResponse {}


pub struct WaypointPush;
impl ros2_client::Service for WaypointPush {
    type Request = WaypointPushRequest;
    type Response = WaypointPushResponse;

    fn request_type_name(&self) -> &str { "WaypointPushRequest" }
    fn response_type_name(&self) -> &str { "WaypointPushResponse" }
}
