use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDelRequest {
    pub id: u32,
    pub url: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
}

impl EndpointDelRequest {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointDelRequest {
    fn default() -> Self {
        EndpointDelRequest {
            id: 0,
            url: ::std::string::String::new(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for EndpointDelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDelResponse {
    pub successful: bool,
}

impl Default for EndpointDelResponse {
    fn default() -> Self {
        EndpointDelResponse {
            successful: false,
        }
    }
}

impl ros2_client::Message for EndpointDelResponse {}


pub struct EndpointDel;
impl ros2_client::Service for EndpointDel {
    type Request = EndpointDelRequest;
    type Response = EndpointDelResponse;

    fn request_type_name(&self) -> &str { "EndpointDelRequest" }
    fn response_type_name(&self) -> &str { "EndpointDelResponse" }
}
