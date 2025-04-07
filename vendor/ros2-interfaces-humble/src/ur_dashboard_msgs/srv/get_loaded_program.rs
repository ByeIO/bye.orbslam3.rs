use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramRequest {

}

impl Default for GetLoadedProgramRequest {
    fn default() -> Self {
        GetLoadedProgramRequest {

        }
    }
}

impl ros2_client::Message for GetLoadedProgramRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramResponse {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub success: bool,
}

impl Default for GetLoadedProgramResponse {
    fn default() -> Self {
        GetLoadedProgramResponse {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetLoadedProgramResponse {}


pub struct GetLoadedProgram;
impl ros2_client::Service for GetLoadedProgram {
    type Request = GetLoadedProgramRequest;
    type Response = GetLoadedProgramResponse;

    fn request_type_name(&self) -> &str { "GetLoadedProgramRequest" }
    fn response_type_name(&self) -> &str { "GetLoadedProgramResponse" }
}
