use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcCustomCmdRequest {
    pub instruction: ::std::string::String,
    pub instruction_type: u8,
    pub motor_num: u8,
    pub value: i32,
}

impl Default for TmcCustomCmdRequest {
    fn default() -> Self {
        TmcCustomCmdRequest {
            instruction: ::std::string::String::new(),
            instruction_type: 0,
            motor_num: 0,
            value: 0,
        }
    }
}

impl ros2_client::Message for TmcCustomCmdRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcCustomCmdResponse {
    pub output: i32,
    pub result: bool,
}

impl Default for TmcCustomCmdResponse {
    fn default() -> Self {
        TmcCustomCmdResponse {
            output: 0,
            result: false,
        }
    }
}

impl ros2_client::Message for TmcCustomCmdResponse {}


pub struct TmcCustomCmd;
impl ros2_client::Service for TmcCustomCmd {
    type Request = TmcCustomCmdRequest;
    type Response = TmcCustomCmdResponse;

    fn request_type_name(&self) -> &str { "TmcCustomCmdRequest" }
    fn response_type_name(&self) -> &str { "TmcCustomCmdResponse" }
}
