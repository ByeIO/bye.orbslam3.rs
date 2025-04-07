use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseRequest {

}

impl Default for PauseRequest {
    fn default() -> Self {
        PauseRequest {

        }
    }
}

impl ros2_client::Message for PauseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseResponse {

}

impl Default for PauseResponse {
    fn default() -> Self {
        PauseResponse {

        }
    }
}

impl ros2_client::Message for PauseResponse {}


pub struct Pause;
impl ros2_client::Service for Pause {
    type Request = PauseRequest;
    type Response = PauseResponse;

    fn request_type_name(&self) -> &str { "PauseRequest" }
    fn response_type_name(&self) -> &str { "PauseResponse" }
}
