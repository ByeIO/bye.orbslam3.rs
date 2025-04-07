use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationRequest {

}

impl Default for HandEyeCalibrationRequest {
    fn default() -> Self {
        HandEyeCalibrationRequest {

        }
    }
}

impl ros2_client::Message for HandEyeCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub error: f64,
    pub translation_error_meter: f64,
    pub rotation_error_degree: f64,
    pub robot_mounted: bool,
}

impl Default for HandEyeCalibrationResponse {
    fn default() -> Self {
        HandEyeCalibrationResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            error: 0.0,
            translation_error_meter: 0.0,
            rotation_error_degree: 0.0,
            robot_mounted: false,
        }
    }
}

impl ros2_client::Message for HandEyeCalibrationResponse {}


pub struct HandEyeCalibration;
impl ros2_client::Service for HandEyeCalibration {
    type Request = HandEyeCalibrationRequest;
    type Response = HandEyeCalibrationResponse;

    fn request_type_name(&self) -> &str { "HandEyeCalibrationRequest" }
    fn response_type_name(&self) -> &str { "HandEyeCalibrationResponse" }
}
