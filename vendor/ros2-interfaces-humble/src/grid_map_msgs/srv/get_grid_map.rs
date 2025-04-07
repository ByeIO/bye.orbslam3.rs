use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapRequest {
    pub frame_id: ::std::string::String,
    pub position_x: f64,
    pub position_y: f64,
    pub length_x: f64,
    pub length_y: f64,
    pub layers: Vec<::std::string::String>,
}

impl Default for GetGridMapRequest {
    fn default() -> Self {
        GetGridMapRequest {
            frame_id: ::std::string::String::new(),
            position_x: 0.0,
            position_y: 0.0,
            length_x: 0.0,
            length_y: 0.0,
            layers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetGridMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapResponse {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for GetGridMapResponse {
    fn default() -> Self {
        GetGridMapResponse {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

impl ros2_client::Message for GetGridMapResponse {}


pub struct GetGridMap;
impl ros2_client::Service for GetGridMap {
    type Request = GetGridMapRequest;
    type Response = GetGridMapResponse;

    fn request_type_name(&self) -> &str { "GetGridMapRequest" }
    fn response_type_name(&self) -> &str { "GetGridMapResponse" }
}
