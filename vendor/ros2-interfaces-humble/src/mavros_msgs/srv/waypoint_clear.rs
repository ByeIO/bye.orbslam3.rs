use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearRequest {

}

impl Default for WaypointClearRequest {
    fn default() -> Self {
        WaypointClearRequest {

        }
    }
}

impl ros2_client::Message for WaypointClearRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearResponse {
    pub success: bool,
}

impl Default for WaypointClearResponse {
    fn default() -> Self {
        WaypointClearResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for WaypointClearResponse {}


pub struct WaypointClear;
impl ros2_client::Service for WaypointClear {
    type Request = WaypointClearRequest;
    type Response = WaypointClearResponse;

    fn request_type_name(&self) -> &str { "WaypointClearRequest" }
    fn response_type_name(&self) -> &str { "WaypointClearResponse" }
}
