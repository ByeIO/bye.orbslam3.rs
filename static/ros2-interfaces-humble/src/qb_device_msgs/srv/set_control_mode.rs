use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetControlModeRequest {
    pub id: i32,
    pub max_repeats: i32,
    pub control: ::std::string::String,
}

impl Default for SetControlModeRequest {
    fn default() -> Self {
        SetControlModeRequest {
            id: 0,
            max_repeats: 0,
            control: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetControlModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetControlModeResponse {
    pub success: bool,
    pub failures: i32,
}

impl Default for SetControlModeResponse {
    fn default() -> Self {
        SetControlModeResponse {
            success: false,
            failures: 0,
        }
    }
}

impl ros2_client::Message for SetControlModeResponse {}


pub struct SetControlMode;
impl ros2_client::Service for SetControlMode {
    type Request = SetControlModeRequest;
    type Response = SetControlModeResponse;

    fn request_type_name(&self) -> &str { "SetControlModeRequest" }
    fn response_type_name(&self) -> &str { "SetControlModeResponse" }
}
