use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceTypeRequest {
    fn default() -> Self {
        ServiceTypeRequest {
            service: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeResponse {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServiceTypeResponse {
    fn default() -> Self {
        ServiceTypeResponse {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceTypeResponse {}


pub struct ServiceType;
impl ros2_client::Service for ServiceType {
    type Request = ServiceTypeRequest;
    type Response = ServiceTypeResponse;

    fn request_type_name(&self) -> &str { "ServiceTypeRequest" }
    fn response_type_name(&self) -> &str { "ServiceTypeResponse" }
}
