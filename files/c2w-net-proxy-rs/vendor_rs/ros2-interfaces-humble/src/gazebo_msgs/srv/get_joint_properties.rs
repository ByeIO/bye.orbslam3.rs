use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetJointPropertiesRequest {
    pub joint_name: ::std::string::String,
}

impl Default for GetJointPropertiesRequest {
    fn default() -> Self {
        GetJointPropertiesRequest {
            joint_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetJointPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetJointPropertiesResponse {
    #[serde(rename = "type")]    pub type_: u8,
    pub damping: Vec<f64>,
    pub position: Vec<f64>,
    pub rate: Vec<f64>,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl GetJointPropertiesResponse {
    pub const REVOLUTE: u8 = 0;
    pub const CONTINUOUS: u8 = 1;
    pub const PRISMATIC: u8 = 2;
    pub const FIXED: u8 = 3;
    pub const BALL: u8 = 4;
    pub const UNIVERSAL: u8 = 5;
}

impl Default for GetJointPropertiesResponse {
    fn default() -> Self {
        GetJointPropertiesResponse {
            type_: 0,
            damping: Vec::new(),
            position: Vec::new(),
            rate: Vec::new(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetJointPropertiesResponse {}


pub struct GetJointProperties;
impl ros2_client::Service for GetJointProperties {
    type Request = GetJointPropertiesRequest;
    type Response = GetJointPropertiesResponse;

    fn request_type_name(&self) -> &str { "GetJointPropertiesRequest" }
    fn response_type_name(&self) -> &str { "GetJointPropertiesResponse" }
}
