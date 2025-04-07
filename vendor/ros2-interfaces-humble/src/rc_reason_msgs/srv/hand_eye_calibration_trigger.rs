use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerRequest {

}

impl Default for HandEyeCalibrationTriggerRequest {
    fn default() -> Self {
        HandEyeCalibrationTriggerRequest {

        }
    }
}

impl ros2_client::Message for HandEyeCalibrationTriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for HandEyeCalibrationTriggerResponse {
    fn default() -> Self {
        HandEyeCalibrationTriggerResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HandEyeCalibrationTriggerResponse {}


pub struct HandEyeCalibrationTrigger;
impl ros2_client::Service for HandEyeCalibrationTrigger {
    type Request = HandEyeCalibrationTriggerRequest;
    type Response = HandEyeCalibrationTriggerResponse;

    fn request_type_name(&self) -> &str { "HandEyeCalibrationTriggerRequest" }
    fn response_type_name(&self) -> &str { "HandEyeCalibrationTriggerResponse" }
}
