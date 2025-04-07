use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapStartRequest {
    pub recording_name: ::std::string::String,
    pub client_map_name: ::std::string::String,
}

impl Default for ClientMapStartRequest {
    fn default() -> Self {
        ClientMapStartRequest {
            recording_name: ::std::string::String::new(),
            client_map_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientMapStartRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapStartResponse {

}

impl Default for ClientMapStartResponse {
    fn default() -> Self {
        ClientMapStartResponse {

        }
    }
}

impl ros2_client::Message for ClientMapStartResponse {}


pub struct ClientMapStart;
impl ros2_client::Service for ClientMapStart {
    type Request = ClientMapStartRequest;
    type Response = ClientMapStartResponse;

    fn request_type_name(&self) -> &str { "ClientMapStartRequest" }
    fn response_type_name(&self) -> &str { "ClientMapStartResponse" }
}
