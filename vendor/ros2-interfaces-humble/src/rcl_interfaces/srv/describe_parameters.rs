use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeParametersRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for DescribeParametersRequest {
    fn default() -> Self {
        DescribeParametersRequest {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for DescribeParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeParametersResponse {
    pub descriptors: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
}

impl Default for DescribeParametersResponse {
    fn default() -> Self {
        DescribeParametersResponse {
            descriptors: Vec::new(),
        }
    }
}

impl ros2_client::Message for DescribeParametersResponse {}


pub struct DescribeParameters;
impl ros2_client::Service for DescribeParameters {
    type Request = DescribeParametersRequest;
    type Response = DescribeParametersResponse;

    fn request_type_name(&self) -> &str { "DescribeParametersRequest" }
    fn response_type_name(&self) -> &str { "DescribeParametersResponse" }
}
