use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkStateRequest {
    pub link_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetLinkStateRequest {
    fn default() -> Self {
        GetLinkStateRequest {
            link_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkStateResponse {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLinkStateResponse {
    fn default() -> Self {
        GetLinkStateResponse {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkStateResponse {}


pub struct GetLinkState;
impl ros2_client::Service for GetLinkState {
    type Request = GetLinkStateRequest;
    type Response = GetLinkStateResponse;

    fn request_type_name(&self) -> &str { "GetLinkStateRequest" }
    fn response_type_name(&self) -> &str { "GetLinkStateResponse" }
}
