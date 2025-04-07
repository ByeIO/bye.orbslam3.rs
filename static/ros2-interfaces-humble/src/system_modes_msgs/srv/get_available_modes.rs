use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesRequest {

}

impl Default for GetAvailableModesRequest {
    fn default() -> Self {
        GetAvailableModesRequest {

        }
    }
}

impl ros2_client::Message for GetAvailableModesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesResponse {
    pub available_modes: Vec<::std::string::String>,
}

impl Default for GetAvailableModesResponse {
    fn default() -> Self {
        GetAvailableModesResponse {
            available_modes: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableModesResponse {}


pub struct GetAvailableModes;
impl ros2_client::Service for GetAvailableModes {
    type Request = GetAvailableModesRequest;
    type Response = GetAvailableModesResponse;

    fn request_type_name(&self) -> &str { "GetAvailableModesRequest" }
    fn response_type_name(&self) -> &str { "GetAvailableModesResponse" }
}
