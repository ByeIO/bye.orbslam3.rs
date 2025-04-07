use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicLandRequest {
    pub enable: bool,
    pub speed_limit: crate::geometry_msgs::msg::Twist,
}

impl Default for DynamicLandRequest {
    fn default() -> Self {
        DynamicLandRequest {
            enable: false,
            speed_limit: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for DynamicLandRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicLandResponse {
    pub success: bool,
}

impl Default for DynamicLandResponse {
    fn default() -> Self {
        DynamicLandResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for DynamicLandResponse {}


pub struct DynamicLand;
impl ros2_client::Service for DynamicLand {
    type Request = DynamicLandRequest;
    type Response = DynamicLandResponse;

    fn request_type_name(&self) -> &str { "DynamicLandRequest" }
    fn response_type_name(&self) -> &str { "DynamicLandResponse" }
}
