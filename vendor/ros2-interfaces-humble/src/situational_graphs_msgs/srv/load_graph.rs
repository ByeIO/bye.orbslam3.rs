use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGraphRequest {
    pub destination: ::std::string::String,
}

impl Default for LoadGraphRequest {
    fn default() -> Self {
        LoadGraphRequest {
            destination: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGraphResponse {
    pub success: bool,
}

impl Default for LoadGraphResponse {
    fn default() -> Self {
        LoadGraphResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for LoadGraphResponse {}


pub struct LoadGraph;
impl ros2_client::Service for LoadGraph {
    type Request = LoadGraphRequest;
    type Response = LoadGraphResponse;

    fn request_type_name(&self) -> &str { "LoadGraphRequest" }
    fn response_type_name(&self) -> &str { "LoadGraphResponse" }
}
