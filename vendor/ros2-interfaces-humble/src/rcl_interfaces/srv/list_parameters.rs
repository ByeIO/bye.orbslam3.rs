use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersRequest {
    pub prefixes: Vec<::std::string::String>,
    pub depth: u64,
}

impl ListParametersRequest {
    pub const DEPTH_RECURSIVE: u64 = 0;
}

impl Default for ListParametersRequest {
    fn default() -> Self {
        ListParametersRequest {
            prefixes: Vec::new(),
            depth: 0,
        }
    }
}

impl ros2_client::Message for ListParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersResponse {
    pub result: crate::rcl_interfaces::msg::ListParametersResult,
}

impl Default for ListParametersResponse {
    fn default() -> Self {
        ListParametersResponse {
            result: crate::rcl_interfaces::msg::ListParametersResult::default(),
        }
    }
}

impl ros2_client::Message for ListParametersResponse {}


pub struct ListParameters;
impl ros2_client::Service for ListParameters {
    type Request = ListParametersRequest;
    type Response = ListParametersResponse;

    fn request_type_name(&self) -> &str { "ListParametersRequest" }
    fn response_type_name(&self) -> &str { "ListParametersResponse" }
}
