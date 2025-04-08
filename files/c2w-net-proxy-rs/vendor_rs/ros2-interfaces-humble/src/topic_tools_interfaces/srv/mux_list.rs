use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListRequest {

}

impl Default for MuxListRequest {
    fn default() -> Self {
        MuxListRequest {

        }
    }
}

impl ros2_client::Message for MuxListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for MuxListResponse {
    fn default() -> Self {
        MuxListResponse {
            topics: Vec::new(),
        }
    }
}

impl ros2_client::Message for MuxListResponse {}


pub struct MuxList;
impl ros2_client::Service for MuxList {
    type Request = MuxListRequest;
    type Response = MuxListResponse;

    fn request_type_name(&self) -> &str { "MuxListRequest" }
    fn response_type_name(&self) -> &str { "MuxListResponse" }
}
