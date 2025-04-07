use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DumpGraphRequest {
    pub destination: ::std::string::String,
}

impl Default for DumpGraphRequest {
    fn default() -> Self {
        DumpGraphRequest {
            destination: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DumpGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DumpGraphResponse {
    pub success: bool,
}

impl Default for DumpGraphResponse {
    fn default() -> Self {
        DumpGraphResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for DumpGraphResponse {}


pub struct DumpGraph;
impl ros2_client::Service for DumpGraph {
    type Request = DumpGraphRequest;
    type Response = DumpGraphResponse;

    fn request_type_name(&self) -> &str { "DumpGraphRequest" }
    fn response_type_name(&self) -> &str { "DumpGraphResponse" }
}
