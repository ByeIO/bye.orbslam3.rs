use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsRequest {

}

impl Default for TopicsRequest {
    fn default() -> Self {
        TopicsRequest {

        }
    }
}

impl ros2_client::Message for TopicsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsResponse {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
}

impl Default for TopicsResponse {
    fn default() -> Self {
        TopicsResponse {
            topics: Vec::new(),
            types: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicsResponse {}


pub struct Topics;
impl ros2_client::Service for Topics {
    type Request = TopicsRequest;
    type Response = TopicsResponse;

    fn request_type_name(&self) -> &str { "TopicsRequest" }
    fn response_type_name(&self) -> &str { "TopicsResponse" }
}
