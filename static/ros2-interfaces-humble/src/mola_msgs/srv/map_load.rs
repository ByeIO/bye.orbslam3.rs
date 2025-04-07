use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapLoadRequest {
    pub map_path: ::std::string::String,
}

impl Default for MapLoadRequest {
    fn default() -> Self {
        MapLoadRequest {
            map_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapLoadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapLoadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapLoadResponse {
    fn default() -> Self {
        MapLoadResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapLoadResponse {}


pub struct MapLoad;
impl ros2_client::Service for MapLoad {
    type Request = MapLoadRequest;
    type Response = MapLoadResponse;

    fn request_type_name(&self) -> &str { "MapLoadRequest" }
    fn response_type_name(&self) -> &str { "MapLoadResponse" }
}
