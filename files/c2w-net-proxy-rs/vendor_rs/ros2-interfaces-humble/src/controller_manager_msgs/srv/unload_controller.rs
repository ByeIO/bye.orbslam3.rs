use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadControllerRequest {
    pub name: ::std::string::String,
}

impl Default for UnloadControllerRequest {
    fn default() -> Self {
        UnloadControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadControllerResponse {
    pub ok: bool,
}

impl Default for UnloadControllerResponse {
    fn default() -> Self {
        UnloadControllerResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for UnloadControllerResponse {}


pub struct UnloadController;
impl ros2_client::Service for UnloadController {
    type Request = UnloadControllerRequest;
    type Response = UnloadControllerResponse;

    fn request_type_name(&self) -> &str { "UnloadControllerRequest" }
    fn response_type_name(&self) -> &str { "UnloadControllerResponse" }
}
