use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackagePickUpRequest {
    pub enable: bool,
    pub speed_limit: crate::geometry_msgs::msg::Twist,
}

impl Default for PackagePickUpRequest {
    fn default() -> Self {
        PackagePickUpRequest {
            enable: false,
            speed_limit: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for PackagePickUpRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackagePickUpResponse {
    pub success: bool,
}

impl Default for PackagePickUpResponse {
    fn default() -> Self {
        PackagePickUpResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for PackagePickUpResponse {}


pub struct PackagePickUp;
impl ros2_client::Service for PackagePickUp {
    type Request = PackagePickUpRequest;
    type Response = PackagePickUpResponse;

    fn request_type_name(&self) -> &str { "PackagePickUpRequest" }
    fn response_type_name(&self) -> &str { "PackagePickUpResponse" }
}
