use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProgramStateRequest {

}

impl Default for GetProgramStateRequest {
    fn default() -> Self {
        GetProgramStateRequest {

        }
    }
}

impl ros2_client::Message for GetProgramStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProgramStateResponse {
    pub state: crate::ur_dashboard_msgs::msg::ProgramState,
    pub program_name: ::std::string::String,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetProgramStateResponse {
    fn default() -> Self {
        GetProgramStateResponse {
            state: crate::ur_dashboard_msgs::msg::ProgramState::default(),
            program_name: ::std::string::String::new(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetProgramStateResponse {}


pub struct GetProgramState;
impl ros2_client::Service for GetProgramState {
    type Request = GetProgramStateRequest;
    type Response = GetProgramStateResponse;

    fn request_type_name(&self) -> &str { "GetProgramStateRequest" }
    fn response_type_name(&self) -> &str { "GetProgramStateResponse" }
}
