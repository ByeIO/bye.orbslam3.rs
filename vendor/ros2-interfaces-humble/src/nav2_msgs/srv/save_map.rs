use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRequest {
    pub map_topic: ::std::string::String,
    pub map_url: ::std::string::String,
    pub image_format: ::std::string::String,
    pub map_mode: ::std::string::String,
    pub free_thresh: f32,
    pub occupied_thresh: f32,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            map_topic: ::std::string::String::new(),
            map_url: ::std::string::String::new(),
            image_format: ::std::string::String::new(),
            map_mode: ::std::string::String::new(),
            free_thresh: 0.0,
            occupied_thresh: 0.0,
        }
    }
}

impl ros2_client::Message for SaveMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapResponse {
    pub result: bool,
}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {
            result: false,
        }
    }
}

impl ros2_client::Message for SaveMapResponse {}


pub struct SaveMap;
impl ros2_client::Service for SaveMap {
    type Request = SaveMapRequest;
    type Response = SaveMapResponse;

    fn request_type_name(&self) -> &str { "SaveMapRequest" }
    fn response_type_name(&self) -> &str { "SaveMapResponse" }
}
