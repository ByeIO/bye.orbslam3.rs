use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2ImageServiceRequest {
    pub request: ::std::string::String,
}

impl Default for ROS2ImageServiceRequest {
    fn default() -> Self {
        ROS2ImageServiceRequest {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ROS2ImageServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2ImageServiceResponse {
    pub response: crate::sensor_msgs::msg::Image,
}

impl Default for ROS2ImageServiceResponse {
    fn default() -> Self {
        ROS2ImageServiceResponse {
            response: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for ROS2ImageServiceResponse {}


pub struct ROS2ImageService;
impl ros2_client::Service for ROS2ImageService {
    type Request = ROS2ImageServiceRequest;
    type Response = ROS2ImageServiceResponse;

    fn request_type_name(&self) -> &str { "ROS2ImageServiceRequest" }
    fn response_type_name(&self) -> &str { "ROS2ImageServiceResponse" }
}
