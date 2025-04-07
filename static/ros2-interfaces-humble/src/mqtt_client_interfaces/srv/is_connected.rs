use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedRequest {

}

impl Default for IsConnectedRequest {
    fn default() -> Self {
        IsConnectedRequest {

        }
    }
}

impl ros2_client::Message for IsConnectedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedResponse {
    pub connected: bool,
}

impl Default for IsConnectedResponse {
    fn default() -> Self {
        IsConnectedResponse {
            connected: false,
        }
    }
}

impl ros2_client::Message for IsConnectedResponse {}


pub struct IsConnected;
impl ros2_client::Service for IsConnected {
    type Request = IsConnectedRequest;
    type Response = IsConnectedResponse;

    fn request_type_name(&self) -> &str { "IsConnectedRequest" }
    fn response_type_name(&self) -> &str { "IsConnectedResponse" }
}
