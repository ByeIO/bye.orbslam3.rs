use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedRequest {

}

impl Default for IsPausedRequest {
    fn default() -> Self {
        IsPausedRequest {

        }
    }
}

impl ros2_client::Message for IsPausedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedResponse {
    pub paused: bool,
}

impl Default for IsPausedResponse {
    fn default() -> Self {
        IsPausedResponse {
            paused: false,
        }
    }
}

impl ros2_client::Message for IsPausedResponse {}


pub struct IsPaused;
impl ros2_client::Service for IsPaused {
    type Request = IsPausedRequest;
    type Response = IsPausedResponse;

    fn request_type_name(&self) -> &str { "IsPausedRequest" }
    fn response_type_name(&self) -> &str { "IsPausedResponse" }
}
