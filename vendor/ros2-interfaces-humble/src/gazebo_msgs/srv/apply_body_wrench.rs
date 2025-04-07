use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyBodyWrenchRequest {
    pub body_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
    pub reference_point: crate::geometry_msgs::msg::Point,
    pub wrench: crate::geometry_msgs::msg::Wrench,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyBodyWrenchRequest {
    fn default() -> Self {
        ApplyBodyWrenchRequest {
            body_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
            reference_point: crate::geometry_msgs::msg::Point::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for ApplyBodyWrenchRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyBodyWrenchResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyBodyWrenchResponse {
    fn default() -> Self {
        ApplyBodyWrenchResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplyBodyWrenchResponse {}


pub struct ApplyBodyWrench;
impl ros2_client::Service for ApplyBodyWrench {
    type Request = ApplyBodyWrenchRequest;
    type Response = ApplyBodyWrenchResponse;

    fn request_type_name(&self) -> &str { "ApplyBodyWrenchRequest" }
    fn response_type_name(&self) -> &str { "ApplyBodyWrenchResponse" }
}
