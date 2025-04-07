use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETRequest {
    pub target: ::std::string::String,
}

impl Default for NovatelFRESETRequest {
    fn default() -> Self {
        NovatelFRESETRequest {
            target: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for NovatelFRESETRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETResponse {
    pub success: bool,
}

impl Default for NovatelFRESETResponse {
    fn default() -> Self {
        NovatelFRESETResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for NovatelFRESETResponse {}


pub struct NovatelFRESET;
impl ros2_client::Service for NovatelFRESET {
    type Request = NovatelFRESETRequest;
    type Response = NovatelFRESETResponse;

    fn request_type_name(&self) -> &str { "NovatelFRESETRequest" }
    fn response_type_name(&self) -> &str { "NovatelFRESETResponse" }
}
