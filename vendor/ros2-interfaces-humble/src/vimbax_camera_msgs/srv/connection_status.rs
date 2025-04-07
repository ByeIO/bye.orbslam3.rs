use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionStatusRequest {

}

impl Default for ConnectionStatusRequest {
    fn default() -> Self {
        ConnectionStatusRequest {

        }
    }
}

impl ros2_client::Message for ConnectionStatusRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionStatusResponse {
    pub connected: bool,
}

impl Default for ConnectionStatusResponse {
    fn default() -> Self {
        ConnectionStatusResponse {
            connected: false,
        }
    }
}

impl ros2_client::Message for ConnectionStatusResponse {}


pub struct ConnectionStatus;
impl ros2_client::Service for ConnectionStatus {
    type Request = ConnectionStatusRequest;
    type Response = ConnectionStatusResponse;

    fn request_type_name(&self) -> &str { "ConnectionStatusRequest" }
    fn response_type_name(&self) -> &str { "ConnectionStatusResponse" }
}
