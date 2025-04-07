use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelStateRequest {
    pub model_name: ::std::string::String,
    pub relative_entity_name: ::std::string::String,
}

impl Default for GetModelStateRequest {
    fn default() -> Self {
        GetModelStateRequest {
            model_name: ::std::string::String::new(),
            relative_entity_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelStateResponse {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetModelStateResponse {
    fn default() -> Self {
        GetModelStateResponse {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelStateResponse {}


pub struct GetModelState;
impl ros2_client::Service for GetModelState {
    type Request = GetModelStateRequest;
    type Response = GetModelStateResponse;

    fn request_type_name(&self) -> &str { "GetModelStateRequest" }
    fn response_type_name(&self) -> &str { "GetModelStateResponse" }
}
