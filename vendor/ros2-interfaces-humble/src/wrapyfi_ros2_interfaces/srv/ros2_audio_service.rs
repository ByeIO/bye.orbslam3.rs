use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2AudioServiceRequest {
    pub request: ::std::string::String,
}

impl Default for ROS2AudioServiceRequest {
    fn default() -> Self {
        ROS2AudioServiceRequest {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ROS2AudioServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2AudioServiceResponse {
    pub response: crate::wrapyfi_ros2_interfaces::msg::ROS2AudioMessage,
}

impl Default for ROS2AudioServiceResponse {
    fn default() -> Self {
        ROS2AudioServiceResponse {
            response: crate::wrapyfi_ros2_interfaces::msg::ROS2AudioMessage::default(),
        }
    }
}

impl ros2_client::Message for ROS2AudioServiceResponse {}


pub struct ROS2AudioService;
impl ros2_client::Service for ROS2AudioService {
    type Request = ROS2AudioServiceRequest;
    type Response = ROS2AudioServiceResponse;

    fn request_type_name(&self) -> &str { "ROS2AudioServiceRequest" }
    fn response_type_name(&self) -> &str { "ROS2AudioServiceResponse" }
}
