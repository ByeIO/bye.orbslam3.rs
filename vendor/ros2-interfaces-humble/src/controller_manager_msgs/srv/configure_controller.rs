use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureControllerRequest {
    pub name: ::std::string::String,
}

impl Default for ConfigureControllerRequest {
    fn default() -> Self {
        ConfigureControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigureControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureControllerResponse {
    pub ok: bool,
}

impl Default for ConfigureControllerResponse {
    fn default() -> Self {
        ConfigureControllerResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for ConfigureControllerResponse {}


pub struct ConfigureController;
impl ros2_client::Service for ConfigureController {
    type Request = ConfigureControllerRequest;
    type Response = ConfigureControllerResponse;

    fn request_type_name(&self) -> &str { "ConfigureControllerRequest" }
    fn response_type_name(&self) -> &str { "ConfigureControllerResponse" }
}
