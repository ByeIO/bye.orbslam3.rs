use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetRequest {
    pub param_id: ::std::string::String,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetRequest {
    fn default() -> Self {
        ParamSetRequest {
            param_id: ::std::string::String::new(),
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetResponse {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetResponse {
    fn default() -> Self {
        ParamSetResponse {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetResponse {}


pub struct ParamSet;
impl ros2_client::Service for ParamSet {
    type Request = ParamSetRequest;
    type Response = ParamSetResponse;

    fn request_type_name(&self) -> &str { "ParamSetRequest" }
    fn response_type_name(&self) -> &str { "ParamSetResponse" }
}
