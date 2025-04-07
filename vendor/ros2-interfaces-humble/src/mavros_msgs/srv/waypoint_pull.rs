use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPullRequest {

}

impl Default for WaypointPullRequest {
    fn default() -> Self {
        WaypointPullRequest {

        }
    }
}

impl ros2_client::Message for WaypointPullRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPullResponse {
    pub success: bool,
    pub wp_received: u32,
}

impl Default for WaypointPullResponse {
    fn default() -> Self {
        WaypointPullResponse {
            success: false,
            wp_received: 0,
        }
    }
}

impl ros2_client::Message for WaypointPullResponse {}


pub struct WaypointPull;
impl ros2_client::Service for WaypointPull {
    type Request = WaypointPullRequest;
    type Response = WaypointPullResponse;

    fn request_type_name(&self) -> &str { "WaypointPullRequest" }
    fn response_type_name(&self) -> &str { "WaypointPullResponse" }
}
