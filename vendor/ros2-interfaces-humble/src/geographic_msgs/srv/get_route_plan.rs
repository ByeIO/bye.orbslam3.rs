use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRoutePlanRequest {
    pub network: crate::unique_identifier_msgs::msg::UUID,
    pub start: crate::unique_identifier_msgs::msg::UUID,
    pub goal: crate::unique_identifier_msgs::msg::UUID,
}

impl Default for GetRoutePlanRequest {
    fn default() -> Self {
        GetRoutePlanRequest {
            network: crate::unique_identifier_msgs::msg::UUID::default(),
            start: crate::unique_identifier_msgs::msg::UUID::default(),
            goal: crate::unique_identifier_msgs::msg::UUID::default(),
        }
    }
}

impl ros2_client::Message for GetRoutePlanRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRoutePlanResponse {
    pub success: bool,
    pub status: ::std::string::String,
    pub plan: crate::geographic_msgs::msg::RoutePath,
}

impl Default for GetRoutePlanResponse {
    fn default() -> Self {
        GetRoutePlanResponse {
            success: false,
            status: ::std::string::String::new(),
            plan: crate::geographic_msgs::msg::RoutePath::default(),
        }
    }
}

impl ros2_client::Message for GetRoutePlanResponse {}


pub struct GetRoutePlan;
impl ros2_client::Service for GetRoutePlan {
    type Request = GetRoutePlanRequest;
    type Response = GetRoutePlanResponse;

    fn request_type_name(&self) -> &str { "GetRoutePlanRequest" }
    fn response_type_name(&self) -> &str { "GetRoutePlanResponse" }
}
