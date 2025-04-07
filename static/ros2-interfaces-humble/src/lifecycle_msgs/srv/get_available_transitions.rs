use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableTransitionsRequest {

}

impl Default for GetAvailableTransitionsRequest {
    fn default() -> Self {
        GetAvailableTransitionsRequest {

        }
    }
}

impl ros2_client::Message for GetAvailableTransitionsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableTransitionsResponse {
    pub available_transitions: Vec<crate::lifecycle_msgs::msg::TransitionDescription>,
}

impl Default for GetAvailableTransitionsResponse {
    fn default() -> Self {
        GetAvailableTransitionsResponse {
            available_transitions: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableTransitionsResponse {}


pub struct GetAvailableTransitions;
impl ros2_client::Service for GetAvailableTransitions {
    type Request = GetAvailableTransitionsRequest;
    type Response = GetAvailableTransitionsResponse;

    fn request_type_name(&self) -> &str { "GetAvailableTransitionsRequest" }
    fn response_type_name(&self) -> &str { "GetAvailableTransitionsResponse" }
}
