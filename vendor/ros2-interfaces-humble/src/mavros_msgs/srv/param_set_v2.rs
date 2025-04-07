use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetV2Request {
    pub force_set: bool,
    pub param_id: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Request {
    fn default() -> Self {
        ParamSetV2Request {
            force_set: false,
            param_id: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetV2Request {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetV2Response {
    pub success: bool,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Response {
    fn default() -> Self {
        ParamSetV2Response {
            success: false,
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetV2Response {}


pub struct ParamSetV2;
impl ros2_client::Service for ParamSetV2 {
    type Request = ParamSetV2Request;
    type Response = ParamSetV2Response;

    fn request_type_name(&self) -> &str { "ParamSetV2Request" }
    fn response_type_name(&self) -> &str { "ParamSetV2Response" }
}
