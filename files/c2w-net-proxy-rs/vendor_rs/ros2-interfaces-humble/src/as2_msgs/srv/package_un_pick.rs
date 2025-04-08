use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageUnPickRequest {
    pub enable: bool,
    pub speed_limit: crate::geometry_msgs::msg::Twist,
}

impl Default for PackageUnPickRequest {
    fn default() -> Self {
        PackageUnPickRequest {
            enable: false,
            speed_limit: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for PackageUnPickRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageUnPickResponse {
    pub success: bool,
}

impl Default for PackageUnPickResponse {
    fn default() -> Self {
        PackageUnPickResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for PackageUnPickResponse {}


pub struct PackageUnPick;
impl ros2_client::Service for PackageUnPick {
    type Request = PackageUnPickRequest;
    type Response = PackageUnPickResponse;

    fn request_type_name(&self) -> &str { "PackageUnPickRequest" }
    fn response_type_name(&self) -> &str { "PackageUnPickResponse" }
}
