use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleInteractiveRequest {

}

impl Default for ToggleInteractiveRequest {
    fn default() -> Self {
        ToggleInteractiveRequest {

        }
    }
}

impl ros2_client::Message for ToggleInteractiveRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleInteractiveResponse {

}

impl Default for ToggleInteractiveResponse {
    fn default() -> Self {
        ToggleInteractiveResponse {

        }
    }
}

impl ros2_client::Message for ToggleInteractiveResponse {}


pub struct ToggleInteractive;
impl ros2_client::Service for ToggleInteractive {
    type Request = ToggleInteractiveRequest;
    type Response = ToggleInteractiveResponse;

    fn request_type_name(&self) -> &str { "ToggleInteractiveRequest" }
    fn response_type_name(&self) -> &str { "ToggleInteractiveResponse" }
}
