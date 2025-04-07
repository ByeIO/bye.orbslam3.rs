use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationRequest {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for GetBasePlaneCalibrationRequest {
    fn default() -> Self {
        GetBasePlaneCalibrationRequest {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for GetBasePlaneCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationResponse {
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetBasePlaneCalibrationResponse {
    fn default() -> Self {
        GetBasePlaneCalibrationResponse {
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetBasePlaneCalibrationResponse {}


pub struct GetBasePlaneCalibration;
impl ros2_client::Service for GetBasePlaneCalibration {
    type Request = GetBasePlaneCalibrationRequest;
    type Response = GetBasePlaneCalibrationResponse;

    fn request_type_name(&self) -> &str { "GetBasePlaneCalibrationRequest" }
    fn response_type_name(&self) -> &str { "GetBasePlaneCalibrationResponse" }
}
