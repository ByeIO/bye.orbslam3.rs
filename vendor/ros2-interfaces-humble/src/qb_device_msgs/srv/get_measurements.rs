use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeasurementsRequest {
    pub id: i32,
    pub max_repeats: i32,
    pub get_positions: bool,
    pub get_currents: bool,
    pub get_commands: bool,
}

impl Default for GetMeasurementsRequest {
    fn default() -> Self {
        GetMeasurementsRequest {
            id: 0,
            max_repeats: 0,
            get_positions: false,
            get_currents: false,
            get_commands: false,
        }
    }
}

impl ros2_client::Message for GetMeasurementsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeasurementsResponse {
    pub success: bool,
    pub failures: i32,
    pub positions: Vec<i16>,
    pub currents: Vec<i16>,
    pub commands: Vec<i16>,
    pub stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for GetMeasurementsResponse {
    fn default() -> Self {
        GetMeasurementsResponse {
            success: false,
            failures: 0,
            positions: Vec::new(),
            currents: Vec::new(),
            commands: Vec::new(),
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
