use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerMapGetImageWithResolutionRequest {
    pub file_name: ::std::string::String,
    pub resolution: u32,
    pub map_name: ::std::string::String,
}

impl Default for ServerMapGetImageWithResolutionRequest {
    fn default() -> Self {
        ServerMapGetImageWithResolutionRequest {
            file_name: ::std::string::String::new(),
            resolution: 0,
            map_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServerMapGetImageWithResolutionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerMapGetImageWithResolutionResponse {

}

impl Default for ServerMapGetImageWithResolutionResponse {
    fn default() -> Self {
        ServerMapGetImageWithResolutionResponse {

        }
    }
}

impl ros2_client::Message for ServerMapGetImageWithResolutionResponse {}


pub struct ServerMapGetImageWithResolution;
impl ros2_client::Service for ServerMapGetImageWithResolution {
    type Request = ServerMapGetImageWithResolutionRequest;
    type Response = ServerMapGetImageWithResolutionResponse;

    fn request_type_name(&self) -> &str { "ServerMapGetImageWithResolutionRequest" }
    fn response_type_name(&self) -> &str { "ServerMapGetImageWithResolutionResponse" }
}
