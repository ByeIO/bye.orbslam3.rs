use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapRequest {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for GetMapRequest {
    fn default() -> Self {
        GetMapRequest {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

impl ros2_client::Message for GetMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapResponse {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMapResponse {
    fn default() -> Self {
        GetMapResponse {
            data: crate::rtabmap_msgs::msg::MapData::default(),
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
