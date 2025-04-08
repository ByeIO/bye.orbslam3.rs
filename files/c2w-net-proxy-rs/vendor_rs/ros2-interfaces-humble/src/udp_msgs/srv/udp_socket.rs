use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSocketRequest {
    pub local_address: ::std::string::String,
    pub local_port: u16,
    pub remote_address: ::std::string::String,
    pub remote_port: u16,
    pub is_broadcast: bool,
}

impl Default for UdpSocketRequest {
    fn default() -> Self {
        UdpSocketRequest {
            local_address: ::std::string::String::new(),
            local_port: 0,
            remote_address: ::std::string::String::new(),
            remote_port: 0,
            is_broadcast: false,
        }
    }
}

impl ros2_client::Message for UdpSocketRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSocketResponse {
    pub socket_created: bool,
}

impl Default for UdpSocketResponse {
    fn default() -> Self {
        UdpSocketResponse {
            socket_created: false,
        }
    }
}

impl ros2_client::Message for UdpSocketResponse {}


pub struct UdpSocket;
impl ros2_client::Service for UdpSocket {
    type Request = UdpSocketRequest;
    type Response = UdpSocketResponse;

    fn request_type_name(&self) -> &str { "UdpSocketRequest" }
    fn response_type_name(&self) -> &str { "UdpSocketResponse" }
}
