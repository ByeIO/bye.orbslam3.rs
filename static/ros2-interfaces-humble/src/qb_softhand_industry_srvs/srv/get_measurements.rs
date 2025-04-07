use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeasurementsRequest {
    pub max_repeats: i32,
    pub get_position: bool,
    pub get_current: bool,
    pub get_velocity: bool,
}

impl Default for GetMeasurementsRequest {
    fn default() -> Self {
        GetMeasurementsRequest {
            max_repeats: 0,
            get_position: false,
            get_current: false,
            get_velocity: false,
        }
    }
}

impl ros2_client::Message for GetMeasurementsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeasurementsResponse {
    pub success: bool,
    pub position: i16,
    pub current: i16,
    pub velocity: i16,
    pub stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for GetMeasurementsResponse {
    fn default() -> Self {
        GetMeasurementsResponse {
            success: false,
            position: 0,
            current: 0,
            velocity: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for GetMeasurementsResponse {}


pub struct GetMeasurements;
impl ros2_client::Service for GetMeasurements {
    type Request = GetMeasurementsRequest;
    type Response = GetMeasurementsResponse;

    fn request_type_name(&self) -> &str { "GetMeasurementsRequest" }
    fn response_type_name(&self) -> &str { "GetMeasurementsResponse" }
}
