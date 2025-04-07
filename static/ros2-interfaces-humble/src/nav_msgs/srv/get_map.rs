use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapRequest {

}

impl Default for GetMapRequest {
    fn default() -> Self {
        GetMapRequest {

        }
    }
}

impl ros2_client::Message for GetMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapResponse {
    fn default() -> Self {
        GetMapResponse {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetMapResponse {}


pub struct GetMap;
impl ros2_client::Service for GetMap {
    type Request = GetMapRequest;
    type Response = GetMapResponse;

    fn request_type_name(&self) -> &str { "GetMapRequest" }
    fn response_type_name(&self) -> &str { "GetMapResponse" }
}
