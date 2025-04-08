use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishersRequest {
    pub topic: ::std::string::String,
}

impl Default for PublishersRequest {
    fn default() -> Self {
        PublishersRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PublishersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishersResponse {
    pub publishers: Vec<::std::string::String>,
}

impl Default for PublishersResponse {
    fn default() -> Self {
        PublishersResponse {
            publishers: Vec::new(),
        }
    }
}

impl ros2_client::Message for PublishersResponse {}


pub struct Publishers;
impl ros2_client::Service for Publishers {
    type Request = PublishersRequest;
    type Response = PublishersResponse;

    fn request_type_name(&self) -> &str { "PublishersRequest" }
    fn response_type_name(&self) -> &str { "PublishersResponse" }
}
