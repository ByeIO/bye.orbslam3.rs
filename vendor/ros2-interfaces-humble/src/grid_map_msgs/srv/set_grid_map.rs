use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapRequest {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for SetGridMapRequest {
    fn default() -> Self {
        SetGridMapRequest {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

impl ros2_client::Message for SetGridMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapResponse {

}

impl Default for SetGridMapResponse {
    fn default() -> Self {
        SetGridMapResponse {

        }
    }
}

impl ros2_client::Message for SetGridMapResponse {}


pub struct SetGridMap;
impl ros2_client::Service for SetGridMap {
    type Request = SetGridMapRequest;
    type Response = SetGridMapResponse;

    fn request_type_name(&self) -> &str { "SetGridMapRequest" }
    fn response_type_name(&self) -> &str { "SetGridMapResponse" }
}
