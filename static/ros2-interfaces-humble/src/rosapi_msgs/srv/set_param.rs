use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamRequest {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for SetParamRequest {
    fn default() -> Self {
        SetParamRequest {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetParamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamResponse {

}

impl Default for SetParamResponse {
    fn default() -> Self {
        SetParamResponse {

        }
    }
}

impl ros2_client::Message for SetParamResponse {}


pub struct SetParam;
impl ros2_client::Service for SetParam {
    type Request = SetParamRequest;
    type Response = SetParamResponse;

    fn request_type_name(&self) -> &str { "SetParamRequest" }
    fn response_type_name(&self) -> &str { "SetParamResponse" }
}
