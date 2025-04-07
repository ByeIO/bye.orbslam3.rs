use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2CompressedImageServiceRequest {
    pub request: ::std::string::String,
}

impl Default for ROS2CompressedImageServiceRequest {
    fn default() -> Self {
        ROS2CompressedImageServiceRequest {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ROS2CompressedImageServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2CompressedImageServiceResponse {
    pub response: crate::sensor_msgs::msg::CompressedImage,
}

impl Default for ROS2CompressedImageServiceResponse {
    fn default() -> Self {
        ROS2CompressedImageServiceResponse {
            response: crate::sensor_msgs::msg::CompressedImage::default(),
        }
    }
}

impl ros2_client::Message for ROS2CompressedImageServiceResponse {}


pub struct ROS2CompressedImageService;
impl ros2_client::Service for ROS2CompressedImageService {
    type Request = ROS2CompressedImageServiceRequest;
    type Response = ROS2CompressedImageServiceResponse;

    fn request_type_name(&self) -> &str { "ROS2CompressedImageServiceRequest" }
    fn response_type_name(&self) -> &str { "ROS2CompressedImageServiceResponse" }
}
