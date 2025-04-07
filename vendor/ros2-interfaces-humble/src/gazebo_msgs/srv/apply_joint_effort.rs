use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyJointEffortRequest {
    pub joint_name: ::std::string::String,
    pub effort: f64,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyJointEffortRequest {
    fn default() -> Self {
        ApplyJointEffortRequest {
            joint_name: ::std::string::String::new(),
            effort: 0.0,
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for ApplyJointEffortRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyJointEffortResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyJointEffortResponse {
    fn default() -> Self {
        ApplyJointEffortResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplyJointEffortResponse {}


pub struct ApplyJointEffort;
impl ros2_client::Service for ApplyJointEffort {
    type Request = ApplyJointEffortRequest;
    type Response = ApplyJointEffortResponse;

    fn request_type_name(&self) -> &str { "ApplyJointEffortRequest" }
    fn response_type_name(&self) -> &str { "ApplyJointEffortResponse" }
}
