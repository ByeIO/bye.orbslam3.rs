use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicFollowerRequest {
    pub enable: bool,
    pub speed_limit: crate::geometry_msgs::msg::Twist,
}

impl Default for DynamicFollowerRequest {
    fn default() -> Self {
        DynamicFollowerRequest {
            enable: false,
            speed_limit: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for DynamicFollowerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicFollowerResponse {
    pub success: bool,
}

impl Default for DynamicFollowerResponse {
    fn default() -> Self {
        DynamicFollowerResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for DynamicFollowerResponse {}


pub struct DynamicFollower;
impl ros2_client::Service for DynamicFollower {
    type Request = DynamicFollowerRequest;
    type Response = DynamicFollowerResponse;

    fn request_type_name(&self) -> &str { "DynamicFollowerRequest" }
    fn response_type_name(&self) -> &str { "DynamicFollowerResponse" }
}
