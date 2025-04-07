use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapListRequest {

}

impl Default for ClientMapListRequest {
    fn default() -> Self {
        ClientMapListRequest {

        }
    }
}

impl ros2_client::Message for ClientMapListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapListResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for ClientMapListResponse {
    fn default() -> Self {
        ClientMapListResponse {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClientMapListResponse {}


pub struct ClientMapList;
impl ros2_client::Service for ClientMapList {
    type Request = ClientMapListRequest;
    type Response = ClientMapListResponse;

    fn request_type_name(&self) -> &str { "ClientMapListRequest" }
    fn response_type_name(&self) -> &str { "ClientMapListResponse" }
}
