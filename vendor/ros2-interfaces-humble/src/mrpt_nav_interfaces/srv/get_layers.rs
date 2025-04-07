use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersRequest {

}

impl Default for GetLayersRequest {
    fn default() -> Self {
        GetLayersRequest {

        }
    }
}

impl ros2_client::Message for GetLayersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersResponse {
    pub layers: Vec<::std::string::String>,
}

impl Default for GetLayersResponse {
    fn default() -> Self {
        GetLayersResponse {
            layers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLayersResponse {}


pub struct GetLayers;
impl ros2_client::Service for GetLayers {
    type Request = GetLayersRequest;
    type Response = GetLayersResponse;

    fn request_type_name(&self) -> &str { "GetLayersRequest" }
    fn response_type_name(&self) -> &str { "GetLayersResponse" }
}
