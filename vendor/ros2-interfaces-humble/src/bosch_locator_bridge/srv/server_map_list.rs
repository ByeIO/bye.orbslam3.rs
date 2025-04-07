use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerMapListRequest {

}

impl Default for ServerMapListRequest {
    fn default() -> Self {
        ServerMapListRequest {

        }
    }
}

impl ros2_client::Message for ServerMapListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerMapListResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for ServerMapListResponse {
    fn default() -> Self {
        ServerMapListResponse {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServerMapListResponse {}


pub struct ServerMapList;
impl ros2_client::Service for ServerMapList {
    type Request = ServerMapListRequest;
    type Response = ServerMapListResponse;

    fn request_type_name(&self) -> &str { "ServerMapListRequest" }
    fn response_type_name(&self) -> &str { "ServerMapListResponse" }
}
