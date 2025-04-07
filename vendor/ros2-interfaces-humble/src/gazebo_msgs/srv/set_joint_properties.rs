use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesRequest {
    pub joint_name: ::std::string::String,
    pub ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties,
}

impl Default for SetJointPropertiesRequest {
    fn default() -> Self {
        SetJointPropertiesRequest {
            joint_name: ::std::string::String::new(),
            ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties::default(),
        }
    }
}

impl ros2_client::Message for SetJointPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointPropertiesResponse {
    fn default() -> Self {
        SetJointPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetJointPropertiesResponse {}


pub struct SetJointProperties;
impl ros2_client::Service for SetJointProperties {
    type Request = SetJointPropertiesRequest;
    type Response = SetJointPropertiesResponse;

    fn request_type_name(&self) -> &str { "SetJointPropertiesRequest" }
    fn response_type_name(&self) -> &str { "SetJointPropertiesResponse" }
}
