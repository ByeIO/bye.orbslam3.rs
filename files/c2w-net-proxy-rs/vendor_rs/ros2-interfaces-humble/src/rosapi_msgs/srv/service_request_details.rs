use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequestDetailsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServiceRequestDetailsRequest {
    fn default() -> Self {
        ServiceRequestDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceRequestDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequestDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceRequestDetailsResponse {
    fn default() -> Self {
        ServiceRequestDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServiceRequestDetailsResponse {}


pub struct ServiceRequestDetails;
impl ros2_client::Service for ServiceRequestDetails {
    type Request = ServiceRequestDetailsRequest;
    type Response = ServiceRequestDetailsResponse;

    fn request_type_name(&self) -> &str { "ServiceRequestDetailsRequest" }
    fn response_type_name(&self) -> &str { "ServiceRequestDetailsResponse" }
}
