use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPhysicsPropertiesRequest {
    pub time_step: f64,
    pub max_update_rate: f64,
    pub gravity: crate::geometry_msgs::msg::Vector3,
    pub ode_config: crate::gazebo_msgs::msg::ODEPhysics,
}

impl Default for SetPhysicsPropertiesRequest {
    fn default() -> Self {
        SetPhysicsPropertiesRequest {
            time_step: 0.0,
            max_update_rate: 0.0,
            gravity: crate::geometry_msgs::msg::Vector3::default(),
            ode_config: crate::gazebo_msgs::msg::ODEPhysics::default(),
        }
    }
}

impl ros2_client::Message for SetPhysicsPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPhysicsPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetPhysicsPropertiesResponse {
    fn default() -> Self {
        SetPhysicsPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetPhysicsPropertiesResponse {}


pub struct SetPhysicsProperties;
impl ros2_client::Service for SetPhysicsProperties {
    type Request = SetPhysicsPropertiesRequest;
    type Response = SetPhysicsPropertiesResponse;

    fn request_type_name(&self) -> &str { "SetPhysicsPropertiesRequest" }
    fn response_type_name(&self) -> &str { "SetPhysicsPropertiesResponse" }
}
