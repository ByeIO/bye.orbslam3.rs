use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapRequest {
    pub map_url: ::std::string::String,
}

impl Default for LoadMapRequest {
    fn default() -> Self {
        LoadMapRequest {
            map_url: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub result: u8,
}

impl LoadMapResponse {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
    pub const RESULT_INVALID_MAP_DATA: u8 = 2;
    pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;
}

impl Default for LoadMapResponse {
    fn default() -> Self {
        LoadMapResponse {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            result: 0,
        }
    }
}

impl ros2_client::Message for LoadMapResponse {}


pub struct LoadMap;
impl ros2_client::Service for LoadMap {
    type Request = LoadMapRequest;
    type Response = LoadMapResponse;

    fn request_type_name(&self) -> &str { "LoadMapRequest" }
    fn response_type_name(&self) -> &str { "LoadMapResponse" }
}
