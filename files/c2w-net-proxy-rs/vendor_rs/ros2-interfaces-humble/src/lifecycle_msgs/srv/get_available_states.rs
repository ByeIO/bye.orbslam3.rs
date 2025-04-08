use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesRequest {

}

impl Default for GetAvailableStatesRequest {
    fn default() -> Self {
        GetAvailableStatesRequest {

        }
    }
}

impl ros2_client::Message for GetAvailableStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesResponse {
    pub available_states: Vec<crate::lifecycle_msgs::msg::State>,
}

impl Default for GetAvailableStatesResponse {
    fn default() -> Self {
        GetAvailableStatesResponse {
            available_states: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableStatesResponse {}


pub struct GetAvailableStates;
impl ros2_client::Service for GetAvailableStates {
    type Request = GetAvailableStatesRequest;
    type Response = GetAvailableStatesResponse;

    fn request_type_name(&self) -> &str { "GetAvailableStatesRequest" }
    fn response_type_name(&self) -> &str { "GetAvailableStatesResponse" }
}
