use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginRequest {

}

impl Default for GetOriginRequest {
    fn default() -> Self {
        GetOriginRequest {

        }
    }
}

impl ros2_client::Message for GetOriginRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginResponse {
    pub success: bool,
    pub origin: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for GetOriginResponse {
    fn default() -> Self {
        GetOriginResponse {
            success: false,
            origin: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for GetOriginResponse {}


pub struct GetOrigin;
impl ros2_client::Service for GetOrigin {
    type Request = GetOriginRequest;
    type Response = GetOriginResponse;

    fn request_type_name(&self) -> &str { "GetOriginRequest" }
    fn response_type_name(&self) -> &str { "GetOriginResponse" }
}
