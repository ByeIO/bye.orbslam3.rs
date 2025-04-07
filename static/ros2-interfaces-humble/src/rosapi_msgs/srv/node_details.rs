use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetailsRequest {
    pub node: ::std::string::String,
}

impl Default for NodeDetailsRequest {
    fn default() -> Self {
        NodeDetailsRequest {
            node: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for NodeDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetailsResponse {
    pub subscribing: Vec<::std::string::String>,
    pub publishing: Vec<::std::string::String>,
    pub services: Vec<::std::string::String>,
}

impl Default for NodeDetailsResponse {
    fn default() -> Self {
        NodeDetailsResponse {
            subscribing: Vec::new(),
            publishing: Vec::new(),
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodeDetailsResponse {}


pub struct NodeDetails;
impl ros2_client::Service for NodeDetails {
    type Request = NodeDetailsRequest;
    type Response = NodeDetailsResponse;

    fn request_type_name(&self) -> &str { "NodeDetailsRequest" }
    fn response_type_name(&self) -> &str { "NodeDetailsResponse" }
}
