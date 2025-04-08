use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationManagerControlProtocolRequest {
    pub request: ::std::string::String,
    pub addition: ::std::string::String,
}

impl StationManagerControlProtocolRequest {
    pub const LOAD: &'static str = "load";
    pub const SAVE: &'static str = "save";
    pub const UPDATE: &'static str = "update";
    pub const ONCE: &'static str = "once";
    pub const CHANGE: &'static str = "change";
}

impl Default for StationManagerControlProtocolRequest {
    fn default() -> Self {
        StationManagerControlProtocolRequest {
            request: ::std::string::String::new(),
            addition: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StationManagerControlProtocolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationManagerControlProtocolResponse {
    pub response: ::std::string::String,
}

impl StationManagerControlProtocolResponse {
    pub const EXECUTED: &'static str = "executed";
    pub const FAILED: &'static str = "failed";
    pub const ERROR: &'static str = "error";
}

impl Default for StationManagerControlProtocolResponse {
    fn default() -> Self {
        StationManagerControlProtocolResponse {
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StationManagerControlProtocolResponse {}


pub struct StationManagerControlProtocol;
impl ros2_client::Service for StationManagerControlProtocol {
    type Request = StationManagerControlProtocolRequest;
    type Response = StationManagerControlProtocolResponse;

    fn request_type_name(&self) -> &str { "StationManagerControlProtocolRequest" }
    fn response_type_name(&self) -> &str { "StationManagerControlProtocolResponse" }
}
