use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateRequest {
    pub name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetEntityStateRequest {
    fn default() -> Self {
        GetEntityStateRequest {
            name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateResponse {
    pub header: crate::std_msgs::msg::Header,
    pub state: crate::gazebo_msgs::msg::EntityState,
    pub success: bool,
}

impl Default for GetEntityStateResponse {
    fn default() -> Self {
        GetEntityStateResponse {
            header: crate::std_msgs::msg::Header::default(),
            state: crate::gazebo_msgs::msg::EntityState::default(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetEntityStateResponse {}


pub struct GetEntityState;
impl ros2_client::Service for GetEntityState {
    type Request = GetEntityStateRequest;
    type Response = GetEntityStateResponse;

    fn request_type_name(&self) -> &str { "GetEntityStateRequest" }
    fn response_type_name(&self) -> &str { "GetEntityStateResponse" }
}
