use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeRequest {
    pub topic: ::std::string::String,
}

impl Default for TopicTypeRequest {
    fn default() -> Self {
        TopicTypeRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TopicTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeResponse {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for TopicTypeResponse {
    fn default() -> Self {
        TopicTypeResponse {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TopicTypeResponse {}


pub struct TopicType;
impl ros2_client::Service for TopicType {
    type Request = TopicTypeRequest;
    type Response = TopicTypeResponse;

    fn request_type_name(&self) -> &str { "TopicTypeRequest" }
    fn response_type_name(&self) -> &str { "TopicTypeResponse" }
}
