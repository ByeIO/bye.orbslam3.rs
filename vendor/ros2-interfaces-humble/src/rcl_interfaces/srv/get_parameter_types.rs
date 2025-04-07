use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterTypesRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParameterTypesRequest {
    fn default() -> Self {
        GetParameterTypesRequest {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParameterTypesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterTypesResponse {
    pub types: Vec<u8>,
}

impl Default for GetParameterTypesResponse {
    fn default() -> Self {
        GetParameterTypesResponse {
            types: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParameterTypesResponse {}


pub struct GetParameterTypes;
impl ros2_client::Service for GetParameterTypes {
    type Request = GetParameterTypesRequest;
    type Response = GetParameterTypesResponse;

    fn request_type_name(&self) -> &str { "GetParameterTypesRequest" }
    fn response_type_name(&self) -> &str { "GetParameterTypesResponse" }
}
