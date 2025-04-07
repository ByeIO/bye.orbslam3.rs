use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersAtomicallyRequest {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersAtomicallyRequest {
    fn default() -> Self {
        SetParametersAtomicallyRequest {
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetParametersAtomicallyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersAtomicallyResponse {
    pub result: crate::rcl_interfaces::msg::SetParametersResult,
}

impl Default for SetParametersAtomicallyResponse {
    fn default() -> Self {
        SetParametersAtomicallyResponse {
            result: crate::rcl_interfaces::msg::SetParametersResult::default(),
        }
    }
}

impl ros2_client::Message for SetParametersAtomicallyResponse {}


pub struct SetParametersAtomically;
impl ros2_client::Service for SetParametersAtomically {
    type Request = SetParametersAtomicallyRequest;
    type Response = SetParametersAtomicallyResponse;

    fn request_type_name(&self) -> &str { "SetParametersAtomicallyRequest" }
    fn response_type_name(&self) -> &str { "SetParametersAtomicallyResponse" }
}
