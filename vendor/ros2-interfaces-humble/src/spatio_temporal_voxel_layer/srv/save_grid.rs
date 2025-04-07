use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGridRequest {
    pub file_name: ::std::string::String,
}

impl Default for SaveGridRequest {
    fn default() -> Self {
        SaveGridRequest {
            file_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveGridRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGridResponse {
    pub map_size_bytes: f64,
    pub status: bool,
}

impl Default for SaveGridResponse {
    fn default() -> Self {
        SaveGridResponse {
            map_size_bytes: 0.0,
            status: false,
        }
    }
}

impl ros2_client::Message for SaveGridResponse {}


pub struct SaveGrid;
impl ros2_client::Service for SaveGrid {
    type Request = SaveGridRequest;
    type Response = SaveGridResponse;

    fn request_type_name(&self) -> &str { "SaveGridRequest" }
    fn response_type_name(&self) -> &str { "SaveGridResponse" }
}
