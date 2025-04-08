use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapSaveRequest {
    pub map_path: ::std::string::String,
}

impl Default for MapSaveRequest {
    fn default() -> Self {
        MapSaveRequest {
            map_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapSaveRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapSaveResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapSaveResponse {
    fn default() -> Self {
        MapSaveResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapSaveResponse {}


pub struct MapSave;
impl ros2_client::Service for MapSave {
    type Request = MapSaveRequest;
    type Response = MapSaveResponse;

    fn request_type_name(&self) -> &str { "MapSaveRequest" }
    fn response_type_name(&self) -> &str { "MapSaveResponse" }
}
