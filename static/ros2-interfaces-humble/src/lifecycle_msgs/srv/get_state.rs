use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateRequest {

}

impl Default for GetStateRequest {
    fn default() -> Self {
        GetStateRequest {

        }
    }
}

impl ros2_client::Message for GetStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateResponse {
    pub current_state: crate::lifecycle_msgs::msg::State,
}

impl Default for GetStateResponse {
    fn default() -> Self {
        GetStateResponse {
            current_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for GetStateResponse {}


pub struct GetState;
impl ros2_client::Service for GetState {
    type Request = GetStateRequest;
    type Response = GetStateResponse;

    fn request_type_name(&self) -> &str { "GetStateRequest" }
    fn response_type_name(&self) -> &str { "GetStateResponse" }
}
