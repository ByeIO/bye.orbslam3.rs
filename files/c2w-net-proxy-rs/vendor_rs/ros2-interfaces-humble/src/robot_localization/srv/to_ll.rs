use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToLLRequest {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for ToLLRequest {
    fn default() -> Self {
        ToLLRequest {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for ToLLRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToLLResponse {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for ToLLResponse {
    fn default() -> Self {
        ToLLResponse {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for ToLLResponse {}


pub struct ToLL;
impl ros2_client::Service for ToLL {
    type Request = ToLLRequest;
    type Response = ToLLResponse;

    fn request_type_name(&self) -> &str { "ToLLRequest" }
    fn response_type_name(&self) -> &str { "ToLLResponse" }
}
