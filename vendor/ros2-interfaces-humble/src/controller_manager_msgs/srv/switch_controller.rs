use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchControllerRequest {
    pub activate_controllers: Vec<::std::string::String>,
    pub deactivate_controllers: Vec<::std::string::String>,
    pub start_controllers: Vec<::std::string::String>,
    pub stop_controllers: Vec<::std::string::String>,
    pub strictness: i32,
    pub start_asap: bool,
    pub activate_asap: bool,
    pub timeout: crate::builtin_interfaces::msg::Duration,
}

impl SwitchControllerRequest {
    pub const BEST_EFFORT: i32 = 1;
    pub const STRICT: i32 = 2;
}

impl Default for SwitchControllerRequest {
    fn default() -> Self {
        SwitchControllerRequest {
            activate_controllers: Vec::new(),
            deactivate_controllers: Vec::new(),
            start_controllers: Vec::new(),
            stop_controllers: Vec::new(),
            strictness: 0,
            start_asap: false,
            activate_asap: false,
            timeout: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for SwitchControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchControllerResponse {
    pub ok: bool,
}

impl Default for SwitchControllerResponse {
    fn default() -> Self {
        SwitchControllerResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for SwitchControllerResponse {}


pub struct SwitchController;
impl ros2_client::Service for SwitchController {
    type Request = SwitchControllerRequest;
    type Response = SwitchControllerResponse;

    fn request_type_name(&self) -> &str { "SwitchControllerRequest" }
    fn response_type_name(&self) -> &str { "SwitchControllerResponse" }
}
