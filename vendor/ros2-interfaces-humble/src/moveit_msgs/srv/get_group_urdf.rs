use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupUrdfRequest {
    pub group_name: ::std::string::String,
}

impl Default for GetGroupUrdfRequest {
    fn default() -> Self {
        GetGroupUrdfRequest {
            group_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGroupUrdfRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupUrdfResponse {
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
    pub urdf_string: ::std::string::String,
}

impl Default for GetGroupUrdfResponse {
    fn default() -> Self {
        GetGroupUrdfResponse {
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
            urdf_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGroupUrdfResponse {}


pub struct GetGroupUrdf;
impl ros2_client::Service for GetGroupUrdf {
    type Request = GetGroupUrdfRequest;
    type Response = GetGroupUrdfResponse;

    fn request_type_name(&self) -> &str { "GetGroupUrdfRequest" }
    fn response_type_name(&self) -> &str { "GetGroupUrdfResponse" }
}
