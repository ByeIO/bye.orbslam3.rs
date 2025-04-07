use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPhysicsPropertiesRequest {

}

impl Default for GetPhysicsPropertiesRequest {
    fn default() -> Self {
        GetPhysicsPropertiesRequest {

        }
    }
}

impl ros2_client::Message for GetPhysicsPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPhysicsPropertiesResponse {
    pub time_step: f64,
    pub pause: bool,
    pub max_update_rate: f64,
    pub gravity: crate::geometry_msgs::msg::Vector3,
    pub ode_config: crate::gazebo_msgs::msg::ODEPhysics,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetPhysicsPropertiesResponse {
    fn default() -> Self {
        GetPhysicsPropertiesResponse {
            time_step: 0.0,
            pause: false,
            max_update_rate: 0.0,
            gravity: crate::geometry_msgs::msg::Vector3::default(),
            ode_config: crate::gazebo_msgs::msg::ODEPhysics::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPhysicsPropertiesResponse {}


pub struct GetPhysicsProperties;
impl ros2_client::Service for GetPhysicsProperties {
    type Request = GetPhysicsPropertiesRequest;
    type Response = GetPhysicsPropertiesResponse;

    fn request_type_name(&self) -> &str { "GetPhysicsPropertiesRequest" }
    fn response_type_name(&self) -> &str { "GetPhysicsPropertiesResponse" }
}
