use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRouteMetadataRequest {
    pub route_guid: ::std::string::String,
    pub metadata_points: Vec<crate::marti_nav_msgs::msg::RoutePoint>,
}

impl Default for UpdateRouteMetadataRequest {
    fn default() -> Self {
        UpdateRouteMetadataRequest {
            route_guid: ::std::string::String::new(),
            metadata_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for UpdateRouteMetadataRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRouteMetadataResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for UpdateRouteMetadataResponse {
    fn default() -> Self {
        UpdateRouteMetadataResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateRouteMetadataResponse {}


pub struct UpdateRouteMetadata;
impl ros2_client::Service for UpdateRouteMetadata {
    type Request = UpdateRouteMetadataRequest;
    type Response = UpdateRouteMetadataResponse;

    fn request_type_name(&self) -> &str { "UpdateRouteMetadataRequest" }
    fn response_type_name(&self) -> &str { "UpdateRouteMetadataResponse" }
}
