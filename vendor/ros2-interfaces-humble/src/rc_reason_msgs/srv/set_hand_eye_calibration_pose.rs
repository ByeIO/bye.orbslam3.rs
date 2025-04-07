use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationPoseRequest {
    pub slot: i32,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetHandEyeCalibrationPoseRequest {
    fn default() -> Self {
        SetHandEyeCalibrationPoseRequest {
            slot: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationPoseResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationPoseResponse {
    fn default() -> Self {
        SetHandEyeCalibrationPoseResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationPoseResponse {}


pub struct SetHandEyeCalibrationPose;
impl ros2_client::Service for SetHandEyeCalibrationPose {
    type Request = SetHandEyeCalibrationPoseRequest;
    type Response = SetHandEyeCalibrationPoseResponse;

    fn request_type_name(&self) -> &str { "SetHandEyeCalibrationPoseRequest" }
    fn response_type_name(&self) -> &str { "SetHandEyeCalibrationPoseResponse" }
}
