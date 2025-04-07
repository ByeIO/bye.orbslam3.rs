use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersRequest {
    pub topic: ::std::string::String,
}

impl Default for SubscribersRequest {
    fn default() -> Self {
        SubscribersRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubscribersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersResponse {
    pub subscribers: Vec<::std::string::String>,
}

impl Default for SubscribersResponse {
    fn default() -> Self {
        SubscribersResponse {
            subscribers: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubscribersResponse {}


pub struct Subscribers;
impl ros2_client::Service for Subscribers {
    type Request = SubscribersRequest;
    type Response = SubscribersResponse;

    fn request_type_name(&self) -> &str { "SubscribersRequest" }
    fn response_type_name(&self) -> &str { "SubscribersResponse" }
}
