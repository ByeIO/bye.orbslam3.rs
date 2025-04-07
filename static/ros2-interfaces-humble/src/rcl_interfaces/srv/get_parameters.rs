use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParametersRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParametersRequest {
    fn default() -> Self {
        GetParametersRequest {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParametersResponse {
    pub values: Vec<crate::rcl_interfaces::msg::ParameterValue>,
}

impl Default for GetParametersResponse {
    fn default() -> Self {
        GetParametersResponse {
            values: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParametersResponse {}


pub struct GetParameters;
impl ros2_client::Service for GetParameters {
    type Request = GetParametersRequest;
    type Response = GetParametersResponse;

    fn request_type_name(&self) -> &str { "GetParametersRequest" }
    fn response_type_name(&self) -> &str { "GetParametersResponse" }
}
