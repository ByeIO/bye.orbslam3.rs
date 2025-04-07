use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRequest {
    pub filename: ::std::string::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapResponse {
    pub success: bool,
}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {
            success: false,
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
