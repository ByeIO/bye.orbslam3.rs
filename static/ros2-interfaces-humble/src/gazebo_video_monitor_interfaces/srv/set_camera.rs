use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraRequest {
    pub camera_name: ::std::string::String,
    pub model_name: ::std::string::String,
    pub link_name: ::std::string::String,
    pub pose: crate::gazebo_video_monitor_interfaces::msg::Pose,
}

impl Default for SetCameraRequest {
    fn default() -> Self {
        SetCameraRequest {
            camera_name: ::std::string::String::new(),
            model_name: ::std::string::String::new(),
            link_name: ::std::string::String::new(),
            pose: crate::gazebo_video_monitor_interfaces::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SetCameraRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraResponse {
    pub message: ::std::string::String,
    pub success: bool,
}

impl Default for SetCameraResponse {
    fn default() -> Self {
        SetCameraResponse {
            message: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for SetCameraResponse {}


pub struct SetCamera;
impl ros2_client::Service for SetCamera {
    type Request = SetCameraRequest;
    type Response = SetCameraResponse;

    fn request_type_name(&self) -> &str { "SetCameraRequest" }
    fn response_type_name(&self) -> &str { "SetCameraResponse" }
}
