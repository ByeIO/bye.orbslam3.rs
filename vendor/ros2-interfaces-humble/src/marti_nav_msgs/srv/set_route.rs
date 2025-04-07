use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRouteRequest {
    pub guid: ::std::string::String,
    pub repeat: bool,
}

impl Default for SetRouteRequest {
    fn default() -> Self {
        SetRouteRequest {
            guid: ::std::string::String::new(),
            repeat: false,
        }
    }
}

impl ros2_client::Message for SetRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetRouteResponse {
    fn default() -> Self {
        SetRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetRouteResponse {}


pub struct SetRoute;
impl ros2_client::Service for SetRoute {
    type Request = SetRouteRequest;
    type Response = SetRouteResponse;

    fn request_type_name(&self) -> &str { "SetRouteRequest" }
    fn response_type_name(&self) -> &str { "SetRouteResponse" }
}
