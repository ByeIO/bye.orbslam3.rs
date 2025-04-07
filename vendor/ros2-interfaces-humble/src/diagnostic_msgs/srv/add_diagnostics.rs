use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDiagnosticsRequest {
    pub load_namespace: ::std::string::String,
}

impl Default for AddDiagnosticsRequest {
    fn default() -> Self {
        AddDiagnosticsRequest {
            load_namespace: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddDiagnosticsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDiagnosticsResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddDiagnosticsResponse {
    fn default() -> Self {
        AddDiagnosticsResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddDiagnosticsResponse {}


pub struct AddDiagnostics;
impl ros2_client::Service for AddDiagnostics {
    type Request = AddDiagnosticsRequest;
    type Response = AddDiagnosticsResponse;

    fn request_type_name(&self) -> &str { "AddDiagnosticsRequest" }
    fn response_type_name(&self) -> &str { "AddDiagnosticsResponse" }
}
