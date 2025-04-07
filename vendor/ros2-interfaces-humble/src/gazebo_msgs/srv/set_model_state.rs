use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelStateRequest {
    pub model_state: crate::gazebo_msgs::msg::ModelState,
}

impl Default for SetModelStateRequest {
    fn default() -> Self {
        SetModelStateRequest {
            model_state: crate::gazebo_msgs::msg::ModelState::default(),
        }
    }
}

impl ros2_client::Message for SetModelStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelStateResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelStateResponse {
    fn default() -> Self {
        SetModelStateResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetModelStateResponse {}


pub struct SetModelState;
impl ros2_client::Service for SetModelState {
    type Request = SetModelStateRequest;
    type Response = SetModelStateResponse;

    fn request_type_name(&self) -> &str { "SetModelStateRequest" }
    fn response_type_name(&self) -> &str { "SetModelStateResponse" }
}
