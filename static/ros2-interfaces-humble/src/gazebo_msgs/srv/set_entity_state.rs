use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateRequest {
    pub state: crate::gazebo_msgs::msg::EntityState,
}

impl Default for SetEntityStateRequest {
    fn default() -> Self {
        SetEntityStateRequest {
            state: crate::gazebo_msgs::msg::EntityState::default(),
        }
    }
}

impl ros2_client::Message for SetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateResponse {
    pub success: bool,
}

impl Default for SetEntityStateResponse {
    fn default() -> Self {
        SetEntityStateResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetEntityStateResponse {}


pub struct SetEntityState;
impl ros2_client::Service for SetEntityState {
    type Request = SetEntityStateRequest;
    type Response = SetEntityStateResponse;

    fn request_type_name(&self) -> &str { "SetEntityStateRequest" }
    fn response_type_name(&self) -> &str { "SetEntityStateResponse" }
}
