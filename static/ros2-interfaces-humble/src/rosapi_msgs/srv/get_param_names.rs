use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesRequest {

}

impl Default for GetParamNamesRequest {
    fn default() -> Self {
        GetParamNamesRequest {

        }
    }
}

impl ros2_client::Message for GetParamNamesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParamNamesResponse {
    fn default() -> Self {
        GetParamNamesResponse {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParamNamesResponse {}


pub struct GetParamNames;
impl ros2_client::Service for GetParamNames {
    type Request = GetParamNamesRequest;
    type Response = GetParamNamesResponse;

    fn request_type_name(&self) -> &str { "GetParamNamesRequest" }
    fn response_type_name(&self) -> &str { "GetParamNamesResponse" }
}
