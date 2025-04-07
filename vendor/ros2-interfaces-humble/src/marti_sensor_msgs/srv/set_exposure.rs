use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureRequest {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureRequest {
    fn default() -> Self {
        SetExposureRequest {
            auto_exposure: false,
            time: 0,
        }
    }
}

impl ros2_client::Message for SetExposureRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureResponse {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureResponse {
    fn default() -> Self {
        SetExposureResponse {
            auto_exposure: false,
            time: 0,
        }
    }
}

impl ros2_client::Message for SetExposureResponse {}


pub struct SetExposure;
impl ros2_client::Service for SetExposure {
    type Request = SetExposureRequest;
    type Response = SetExposureResponse;

    fn request_type_name(&self) -> &str { "SetExposureRequest" }
    fn response_type_name(&self) -> &str { "SetExposureResponse" }
}
