use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGeographicMapRequest {
    pub updates: crate::geographic_msgs::msg::GeographicMapChanges,
}

impl Default for UpdateGeographicMapRequest {
    fn default() -> Self {
        UpdateGeographicMapRequest {
            updates: crate::geographic_msgs::msg::GeographicMapChanges::default(),
        }
    }
}

impl ros2_client::Message for UpdateGeographicMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGeographicMapResponse {
    pub success: bool,
    pub status: ::std::string::String,
}

impl Default for UpdateGeographicMapResponse {
    fn default() -> Self {
        UpdateGeographicMapResponse {
            success: false,
            status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateGeographicMapResponse {}


pub struct UpdateGeographicMap;
impl ros2_client::Service for UpdateGeographicMap {
    type Request = UpdateGeographicMapRequest;
    type Response = UpdateGeographicMapResponse;

    fn request_type_name(&self) -> &str { "UpdateGeographicMapRequest" }
    fn response_type_name(&self) -> &str { "UpdateGeographicMapResponse" }
}
