use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsRequest {

}

impl Default for ListLabelsRequest {
    fn default() -> Self {
        ListLabelsRequest {

        }
    }
}

impl ros2_client::Message for ListLabelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsResponse {
    pub ids: Vec<i32>,
    pub labels: Vec<::std::string::String>,
}

impl Default for ListLabelsResponse {
    fn default() -> Self {
        ListLabelsResponse {
            ids: Vec::new(),
            labels: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListLabelsResponse {}


pub struct ListLabels;
impl ros2_client::Service for ListLabels {
    type Request = ListLabelsRequest;
    type Response = ListLabelsResponse;

    fn request_type_name(&self) -> &str { "ListLabelsRequest" }
    fn response_type_name(&self) -> &str { "ListLabelsResponse" }
}
