use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamRequest {
    pub name: ::std::string::String,
    pub default_value: ::std::string::String,
}

impl Default for GetParamRequest {
    fn default() -> Self {
        GetParamRequest {
            name: ::std::string::String::new(),
            default_value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetParamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamResponse {
    pub value: ::std::string::String,
}

impl Default for GetParamResponse {
    fn default() -> Self {
        GetParamResponse {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetParamResponse {}


pub struct GetParam;
impl ros2_client::Service for GetParam {
    type Request = GetParamRequest;
    type Response = GetParamResponse;

    fn request_type_name(&self) -> &str { "GetParamRequest" }
    fn response_type_name(&self) -> &str { "GetParamResponse" }
}
