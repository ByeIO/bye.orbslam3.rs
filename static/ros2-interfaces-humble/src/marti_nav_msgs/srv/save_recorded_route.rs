use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRecordedRouteRequest {
    pub name: ::std::string::String,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRecordedRouteRequest {
    fn default() -> Self {
        SaveRecordedRouteRequest {
            name: ::std::string::String::new(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRecordedRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRecordedRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRecordedRouteResponse {
    fn default() -> Self {
        SaveRecordedRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRecordedRouteResponse {}


pub struct SaveRecordedRoute;
impl ros2_client::Service for SaveRecordedRoute {
    type Request = SaveRecordedRouteRequest;
    type Response = SaveRecordedRouteResponse;

    fn request_type_name(&self) -> &str { "SaveRecordedRouteRequest" }
    fn response_type_name(&self) -> &str { "SaveRecordedRouteResponse" }
}
