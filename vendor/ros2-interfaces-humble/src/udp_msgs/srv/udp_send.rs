use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSendRequest {
    pub local_address: ::std::string::String,
    pub local_port: u16,
    pub remote_address: ::std::string::String,
    pub remote_port: u16,
    pub data: Vec<u8>,
}

impl Default for UdpSendRequest {
    fn default() -> Self {
        UdpSendRequest {
            local_address: ::std::string::String::new(),
            local_port: 0,
            remote_address: ::std::string::String::new(),
            remote_port: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UdpSendRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSendResponse {
    pub sent: bool,
}

impl Default for UdpSendResponse {
    fn default() -> Self {
        UdpSendResponse {
            sent: false,
        }
    }
}

impl ros2_client::Message for UdpSendResponse {}


pub struct UdpSend;
impl ros2_client::Service for UdpSend {
    type Request = UdpSendRequest;
    type Response = UdpSendResponse;

    fn request_type_name(&self) -> &str { "UdpSendRequest" }
    fn response_type_name(&self) -> &str { "UdpSendResponse" }
}
