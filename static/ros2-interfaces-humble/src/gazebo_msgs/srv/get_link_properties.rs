use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkPropertiesRequest {
    pub link_name: ::std::string::String,
}

impl Default for GetLinkPropertiesRequest {
    fn default() -> Self {
        GetLinkPropertiesRequest {
            link_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkPropertiesResponse {
    pub com: crate::geometry_msgs::msg::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLinkPropertiesResponse {
    fn default() -> Self {
        GetLinkPropertiesResponse {
            com: crate::geometry_msgs::msg::Pose::default(),
            gravity_mode: false,
            mass: 0.0,
            ixx: 0.0,
            ixy: 0.0,
            ixz: 0.0,
            iyy: 0.0,
            iyz: 0.0,
            izz: 0.0,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkPropertiesResponse {}


pub struct GetLinkProperties;
impl ros2_client::Service for GetLinkProperties {
    type Request = GetLinkPropertiesRequest;
    type Response = GetLinkPropertiesResponse;

    fn request_type_name(&self) -> &str { "GetLinkPropertiesRequest" }
    fn response_type_name(&self) -> &str { "GetLinkPropertiesResponse" }
}
