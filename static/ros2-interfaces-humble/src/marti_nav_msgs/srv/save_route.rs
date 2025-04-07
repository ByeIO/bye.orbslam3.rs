use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRouteRequest {
    pub name: ::std::string::String,
    pub guid: ::std::string::String,
    pub route: crate::marti_nav_msgs::msg::Route,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRouteRequest {
    fn default() -> Self {
        SaveRouteRequest {
            name: ::std::string::String::new(),
            guid: ::std::string::String::new(),
            route: crate::marti_nav_msgs::msg::Route::default(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRouteResponse {
    fn default() -> Self {
        SaveRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRouteResponse {}


pub struct SaveRoute;
impl ros2_client::Service for SaveRoute {
    type Request = SaveRouteRequest;
    type Response = SaveRouteResponse;

    fn request_type_name(&self) -> &str { "SaveRouteRequest" }
    fn response_type_name(&self) -> &str { "SaveRouteResponse" }
}
