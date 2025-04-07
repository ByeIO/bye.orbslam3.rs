use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGoHomeAltitudeRequest {

}

impl Default for GetGoHomeAltitudeRequest {
    fn default() -> Self {
        GetGoHomeAltitudeRequest {

        }
    }
}

impl ros2_client::Message for GetGoHomeAltitudeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGoHomeAltitudeResponse {
    pub altitude: u16,
    pub success: bool,
}

impl Default for GetGoHomeAltitudeResponse {
    fn default() -> Self {
        GetGoHomeAltitudeResponse {
            altitude: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for GetGoHomeAltitudeResponse {}


pub struct GetGoHomeAltitude;
impl ros2_client::Service for GetGoHomeAltitude {
    type Request = GetGoHomeAltitudeRequest;
    type Response = GetGoHomeAltitudeResponse;

    fn request_type_name(&self) -> &str { "GetGoHomeAltitudeRequest" }
    fn response_type_name(&self) -> &str { "GetGoHomeAltitudeResponse" }
}
