use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelRequest {
    pub label: ::std::string::String,
}

impl Default for RemoveLabelRequest {
    fn default() -> Self {
        RemoveLabelRequest {
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RemoveLabelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelResponse {

}

impl Default for RemoveLabelResponse {
    fn default() -> Self {
        RemoveLabelResponse {

        }
    }
}

impl ros2_client::Message for RemoveLabelResponse {}


pub struct RemoveLabel;
impl ros2_client::Service for RemoveLabel {
    type Request = RemoveLabelRequest;
    type Response = RemoveLabelResponse;

    fn request_type_name(&self) -> &str { "RemoveLabelRequest" }
    fn response_type_name(&self) -> &str { "RemoveLabelResponse" }
}
