use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedImageCropRequest {
    pub top_left: crate::geometry_msgs::msg::Pose2D,
    pub bottom_right: crate::geometry_msgs::msg::Pose2D,
}

impl Default for NormalizedImageCropRequest {
    fn default() -> Self {
        NormalizedImageCropRequest {
            top_left: crate::geometry_msgs::msg::Pose2D::default(),
            bottom_right: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}

impl ros2_client::Message for NormalizedImageCropRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedImageCropResponse {
    pub status: i64,
}

impl Default for NormalizedImageCropResponse {
    fn default() -> Self {
        NormalizedImageCropResponse {
            status: 0,
        }
    }
}

impl ros2_client::Message for NormalizedImageCropResponse {}


pub struct NormalizedImageCrop;
impl ros2_client::Service for NormalizedImageCrop {
    type Request = NormalizedImageCropRequest;
    type Response = NormalizedImageCropResponse;

    fn request_type_name(&self) -> &str { "NormalizedImageCropRequest" }
    fn response_type_name(&self) -> &str { "NormalizedImageCropResponse" }
}
