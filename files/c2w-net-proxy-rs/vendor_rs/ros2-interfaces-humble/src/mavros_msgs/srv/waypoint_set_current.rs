use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointSetCurrentRequest {
    pub wp_seq: u16,
}

impl Default for WaypointSetCurrentRequest {
    fn default() -> Self {
        WaypointSetCurrentRequest {
            wp_seq: 0,
        }
    }
}

impl ros2_client::Message for WaypointSetCurrentRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointSetCurrentResponse {
    pub success: bool,
}

impl Default for WaypointSetCurrentResponse {
    fn default() -> Self {
        WaypointSetCurrentResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for WaypointSetCurrentResponse {}


pub struct WaypointSetCurrent;
impl ros2_client::Service for WaypointSetCurrent {
    type Request = WaypointSetCurrentRequest;
    type Response = WaypointSetCurrentResponse;

    fn request_type_name(&self) -> &str { "WaypointSetCurrentRequest" }
    fn response_type_name(&self) -> &str { "WaypointSetCurrentResponse" }
}
