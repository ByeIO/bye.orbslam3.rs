use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPathValidRequest {
    pub path: crate::nav_msgs::msg::Path,
}

impl Default for IsPathValidRequest {
    fn default() -> Self {
        IsPathValidRequest {
            path: crate::nav_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for IsPathValidRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPathValidResponse {
    pub is_valid: bool,
    pub invalid_pose_indices: Vec<i32>,
}

impl Default for IsPathValidResponse {
    fn default() -> Self {
        IsPathValidResponse {
            is_valid: false,
            invalid_pose_indices: Vec::new(),
        }
    }
}

impl ros2_client::Message for IsPathValidResponse {}


pub struct IsPathValid;
impl ros2_client::Service for IsPathValid {
    type Request = IsPathValidRequest;
    type Response = IsPathValidResponse;

    fn request_type_name(&self) -> &str { "IsPathValidRequest" }
    fn response_type_name(&self) -> &str { "IsPathValidResponse" }
}
