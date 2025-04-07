use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesRequest {

}

impl Default for NodesRequest {
    fn default() -> Self {
        NodesRequest {

        }
    }
}

impl ros2_client::Message for NodesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<::std::string::String>,
}

impl Default for NodesResponse {
    fn default() -> Self {
        NodesResponse {
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodesResponse {}


pub struct Nodes;
impl ros2_client::Service for Nodes {
    type Request = NodesRequest;
    type Response = NodesResponse;

    fn request_type_name(&self) -> &str { "NodesRequest" }
    fn response_type_name(&self) -> &str { "NodesResponse" }
}
