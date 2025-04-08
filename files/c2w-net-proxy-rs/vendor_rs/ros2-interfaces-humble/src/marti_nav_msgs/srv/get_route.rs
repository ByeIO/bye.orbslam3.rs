use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteRequest {
    pub guid: ::std::string::String,
}

impl Default for GetRouteRequest {
    fn default() -> Self {
        GetRouteRequest {
            guid: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteResponse {
    pub route: crate::marti_nav_msgs::msg::Route,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteResponse {
    fn default() -> Self {
        GetRouteResponse {
            route: crate::marti_nav_msgs::msg::Route::default(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRouteResponse {}


pub struct GetRoute;
impl ros2_client::Service for GetRoute {
    type Request = GetRouteRequest;
    type Response = GetRouteResponse;

    fn request_type_name(&self) -> &str { "GetRouteRequest" }
    fn response_type_name(&self) -> &str { "GetRouteResponse" }
}
