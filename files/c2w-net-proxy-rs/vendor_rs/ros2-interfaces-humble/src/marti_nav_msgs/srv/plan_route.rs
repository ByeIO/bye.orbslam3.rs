use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRouteRequest {
    pub header: crate::std_msgs::msg::Header,
    pub waypoints: Vec<crate::geometry_msgs::msg::Pose>,
    pub plan_from_vehicle: bool,
}

impl Default for PlanRouteRequest {
    fn default() -> Self {
        PlanRouteRequest {
            header: crate::std_msgs::msg::Header::default(),
            waypoints: Vec::new(),
            plan_from_vehicle: false,
        }
    }
}

impl ros2_client::Message for PlanRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRouteResponse {
    pub route: crate::marti_nav_msgs::msg::Route,
    pub success: bool,
    pub message: ::std::string::String,
    pub cost: f64,
}

impl Default for PlanRouteResponse {
    fn default() -> Self {
        PlanRouteResponse {
            route: crate::marti_nav_msgs::msg::Route::default(),
            success: false,
            message: ::std::string::String::new(),
            cost: 0.0,
        }
    }
}

impl ros2_client::Message for PlanRouteResponse {}


pub struct PlanRoute;
impl ros2_client::Service for PlanRoute {
    type Request = PlanRouteRequest;
    type Response = PlanRouteResponse;

    fn request_type_name(&self) -> &str { "PlanRouteRequest" }
    fn response_type_name(&self) -> &str { "PlanRouteResponse" }
}
