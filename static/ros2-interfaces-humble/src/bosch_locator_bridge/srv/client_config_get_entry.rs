use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientConfigGetEntryRequest {
    pub name: ::std::string::String,
}

impl Default for ClientConfigGetEntryRequest {
    fn default() -> Self {
        ClientConfigGetEntryRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientConfigGetEntryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientConfigGetEntryResponse {
    pub value: ::std::string::String,
}

impl Default for ClientConfigGetEntryResponse {
    fn default() -> Self {
        ClientConfigGetEntryResponse {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientConfigGetEntryResponse {}


pub struct ClientConfigGetEntry;
impl ros2_client::Service for ClientConfigGetEntry {
    type Request = ClientConfigGetEntryRequest;
    type Response = ClientConfigGetEntryResponse;

    fn request_type_name(&self) -> &str { "ClientConfigGetEntryRequest" }
    fn response_type_name(&self) -> &str { "ClientConfigGetEntryResponse" }
}
