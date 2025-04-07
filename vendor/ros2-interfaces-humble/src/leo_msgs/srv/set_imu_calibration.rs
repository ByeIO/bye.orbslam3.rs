use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetImuCalibrationRequest {
    pub gyro_bias_x: f64,
    pub gyro_bias_y: f64,
    pub gyro_bias_z: f64,
}

impl Default for SetImuCalibrationRequest {
    fn default() -> Self {
        SetImuCalibrationRequest {
            gyro_bias_x: 0.0,
            gyro_bias_y: 0.0,
            gyro_bias_z: 0.0,
        }
    }
}

impl ros2_client::Message for SetImuCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetImuCalibrationResponse {
    pub success: bool,
}

impl Default for SetImuCalibrationResponse {
    fn default() -> Self {
        SetImuCalibrationResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetImuCalibrationResponse {}


pub struct SetImuCalibration;
impl ros2_client::Service for SetImuCalibration {
    type Request = SetImuCalibrationRequest;
    type Response = SetImuCalibrationResponse;

    fn request_type_name(&self) -> &str { "SetImuCalibrationRequest" }
    fn response_type_name(&self) -> &str { "SetImuCalibrationResponse" }
}
