use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializePoseGraphRequest {
    pub filename: ::std::string::String,
}

impl Default for SerializePoseGraphRequest {
    fn default() -> Self {
        SerializePoseGraphRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SerializePoseGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializePoseGraphResponse {
    pub result: u8,
}

impl SerializePoseGraphResponse {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_FAILED_TO_WRITE_FILE: u8 = 255;
}

impl Default for SerializePoseGraphResponse {
    fn default() -> Self {
        SerializePoseGraphResponse {
            result: 0,
        }
    }
}

impl ros2_client::Message for SerializePoseGraphResponse {}


pub struct SerializePoseGraph;
impl ros2_client::Service for SerializePoseGraph {
    type Request = SerializePoseGraphRequest;
    type Response = SerializePoseGraphResponse;

    fn request_type_name(&self) -> &str { "SerializePoseGraphRequest" }
    fn response_type_name(&self) -> &str { "SerializePoseGraphResponse" }
}
