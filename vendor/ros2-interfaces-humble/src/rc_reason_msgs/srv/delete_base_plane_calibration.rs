use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationRequest {

}

impl Default for DeleteBasePlaneCalibrationRequest {
    fn default() -> Self {
        DeleteBasePlaneCalibrationRequest {

        }
    }
}

impl ros2_client::Message for DeleteBasePlaneCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteBasePlaneCalibrationResponse {
    fn default() -> Self {
        DeleteBasePlaneCalibrationResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteBasePlaneCalibrationResponse {}


pub struct DeleteBasePlaneCalibration;
impl ros2_client::Service for DeleteBasePlaneCalibration {
    type Request = DeleteBasePlaneCalibrationRequest;
    type Response = DeleteBasePlaneCalibrationResponse;

    fn request_type_name(&self) -> &str { "DeleteBasePlaneCalibrationRequest" }
    fn response_type_name(&self) -> &str { "DeleteBasePlaneCalibrationResponse" }
}
