use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2NativeObjectServiceRequest {
    pub request: ::std::string::String,
}

impl Default for ROS2NativeObjectServiceRequest {
    fn default() -> Self {
        ROS2NativeObjectServiceRequest {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ROS2NativeObjectServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROS2NativeObjectServiceResponse {
    pub response: ::std::string::String,
}

impl Default for ROS2NativeObjectServiceResponse {
    fn default() -> Self {
        ROS2NativeObjectServiceResponse {
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ROS2NativeObjectServiceResponse {}


pub struct ROS2NativeObjectService;
impl ros2_client::Service for ROS2NativeObjectService {
    type Request = ROS2NativeObjectServiceRequest;
    type Response = ROS2NativeObjectServiceResponse;

    fn request_type_name(&self) -> &str { "ROS2NativeObjectServiceRequest" }
    fn response_type_name(&self) -> &str { "ROS2NativeObjectServiceResponse" }
}
