use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerRequest {
    pub name: ::std::string::String,
}

impl Default for LoadControllerRequest {
    fn default() -> Self {
        LoadControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerResponse {
    pub ok: bool,
}

impl Default for LoadControllerResponse {
    fn default() -> Self {
        LoadControllerResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for LoadControllerResponse {}


pub struct LoadController;
impl ros2_client::Service for LoadController {
    type Request = LoadControllerRequest;
    type Response = LoadControllerResponse;

    fn request_type_name(&self) -> &str { "LoadControllerRequest" }
    fn response_type_name(&self) -> &str { "LoadControllerResponse" }
}
