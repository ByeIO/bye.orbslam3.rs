use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServicesForTypeRequest {
    fn default() -> Self {
        ServicesForTypeRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServicesForTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesForTypeResponse {
    fn default() -> Self {
        ServicesForTypeResponse {
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServicesForTypeResponse {}


pub struct ServicesForType;
impl ros2_client::Service for ServicesForType {
    type Request = ServicesForTypeRequest;
    type Response = ServicesForTypeResponse;

    fn request_type_name(&self) -> &str { "ServicesForTypeRequest" }
    fn response_type_name(&self) -> &str { "ServicesForTypeResponse" }
}
