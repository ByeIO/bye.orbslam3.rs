use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetRequest {
    pub param_id: ::std::string::String,
}

impl Default for ParamGetRequest {
    fn default() -> Self {
        ParamGetRequest {
            param_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ParamGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetResponse {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamGetResponse {
    fn default() -> Self {
        ParamGetResponse {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamGetResponse {}


pub struct ParamGet;
impl ros2_client::Service for ParamGet {
    type Request = ParamGetRequest;
    type Response = ParamGetResponse;

    fn request_type_name(&self) -> &str { "ParamGetRequest" }
    fn response_type_name(&self) -> &str { "ParamGetResponse" }
}
