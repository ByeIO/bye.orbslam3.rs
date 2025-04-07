use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub robot_mounted: bool,
}

impl Default for SetHandEyeCalibrationRequest {
    fn default() -> Self {
        SetHandEyeCalibrationRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
            robot_mounted: false,
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationResponse {
    fn default() -> Self {
        SetHandEyeCalibrationResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationResponse {}


pub struct SetHandEyeCalibration;
impl ros2_client::Service for SetHandEyeCalibration {
    type Request = SetHandEyeCalibrationRequest;
    type Response = SetHandEyeCalibrationResponse;

    fn request_type_name(&self) -> &str { "SetHandEyeCalibrationRequest" }
    fn response_type_name(&self) -> &str { "SetHandEyeCalibrationResponse" }
}
