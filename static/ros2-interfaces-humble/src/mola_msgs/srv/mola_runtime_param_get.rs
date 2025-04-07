use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolaRuntimeParamGetRequest {

}

impl Default for MolaRuntimeParamGetRequest {
    fn default() -> Self {
        MolaRuntimeParamGetRequest {

        }
    }
}

impl ros2_client::Message for MolaRuntimeParamGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolaRuntimeParamGetResponse {
    pub parameters: ::std::string::String,
}

impl Default for MolaRuntimeParamGetResponse {
    fn default() -> Self {
        MolaRuntimeParamGetResponse {
            parameters: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MolaRuntimeParamGetResponse {}


pub struct MolaRuntimeParamGet;
impl ros2_client::Service for MolaRuntimeParamGet {
    type Request = MolaRuntimeParamGetRequest;
    type Response = MolaRuntimeParamGetResponse;

    fn request_type_name(&self) -> &str { "MolaRuntimeParamGetRequest" }
    fn response_type_name(&self) -> &str { "MolaRuntimeParamGetResponse" }
}
