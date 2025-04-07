use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsAndRawTypesRequest {

}

impl Default for TopicsAndRawTypesRequest {
    fn default() -> Self {
        TopicsAndRawTypesRequest {

        }
    }
}

impl ros2_client::Message for TopicsAndRawTypesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsAndRawTypesResponse {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
    pub typedefs_full_text: Vec<::std::string::String>,
}

impl Default for TopicsAndRawTypesResponse {
    fn default() -> Self {
        TopicsAndRawTypesResponse {
            topics: Vec::new(),
            types: Vec::new(),
            typedefs_full_text: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicsAndRawTypesResponse {}


pub struct TopicsAndRawTypes;
impl ros2_client::Service for TopicsAndRawTypes {
    type Request = TopicsAndRawTypesRequest;
    type Response = TopicsAndRawTypesResponse;

    fn request_type_name(&self) -> &str { "TopicsAndRawTypesRequest" }
    fn response_type_name(&self) -> &str { "TopicsAndRawTypesResponse" }
}
