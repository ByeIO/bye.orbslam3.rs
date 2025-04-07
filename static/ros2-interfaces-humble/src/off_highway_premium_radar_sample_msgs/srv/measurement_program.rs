use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementProgramRequest {
    pub id: u16,
}

impl MeasurementProgramRequest {
    pub const DMP00: u16 = 0;
    pub const DMP01: u16 = 1;
    pub const DMP02: u16 = 2;
    pub const DMP04: u16 = 4;
}

impl Default for MeasurementProgramRequest {
    fn default() -> Self {
        MeasurementProgramRequest {
            id: 0,
        }
    }
}

impl ros2_client::Message for MeasurementProgramRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementProgramResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for MeasurementProgramResponse {
    fn default() -> Self {
        MeasurementProgramResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MeasurementProgramResponse {}


pub struct MeasurementProgram;
impl ros2_client::Service for MeasurementProgram {
    type Request = MeasurementProgramRequest;
    type Response = MeasurementProgramResponse;

    fn request_type_name(&self) -> &str { "MeasurementProgramRequest" }
    fn response_type_name(&self) -> &str { "MeasurementProgramResponse" }
}
