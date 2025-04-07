use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridmapLayerRequest {
    pub layer_name: ::std::string::String,
}

impl Default for GetGridmapLayerRequest {
    fn default() -> Self {
        GetGridmapLayerRequest {
            layer_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGridmapLayerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridmapLayerResponse {
    pub valid: bool,
    pub grid: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetGridmapLayerResponse {
    fn default() -> Self {
        GetGridmapLayerResponse {
            valid: false,
            grid: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetGridmapLayerResponse {}


pub struct GetGridmapLayer;
impl ros2_client::Service for GetGridmapLayer {
    type Request = GetGridmapLayerRequest;
    type Response = GetGridmapLayerResponse;

    fn request_type_name(&self) -> &str { "GetGridmapLayerRequest" }
    fn response_type_name(&self) -> &str { "GetGridmapLayerResponse" }
}
