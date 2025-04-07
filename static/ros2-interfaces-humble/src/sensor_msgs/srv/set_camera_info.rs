use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraInfoRequest {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
}

impl Default for SetCameraInfoRequest {
    fn default() -> Self {
        SetCameraInfoRequest {
            camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
        }
    }
}

impl ros2_client::Message for SetCameraInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraInfoResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetCameraInfoResponse {
    fn default() -> Self {
        SetCameraInfoResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetCameraInfoResponse {}


pub struct SetCameraInfo;
impl ros2_client::Service for SetCameraInfo {
    type Request = SetCameraInfoRequest;
    type Response = SetCameraInfoResponse;

    fn request_type_name(&self) -> &str { "SetCameraInfoRequest" }
    fn response_type_name(&self) -> &str { "SetCameraInfoResponse" }
}
