use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapRequest {

}

impl Default for GetPointMapRequest {
    fn default() -> Self {
        GetPointMapRequest {

        }
    }
}

impl ros2_client::Message for GetPointMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapResponse {
    pub map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapResponse {
    fn default() -> Self {
        GetPointMapResponse {
            map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for GetPointMapResponse {}


pub struct GetPointMap;
impl ros2_client::Service for GetPointMap {
    type Request = GetPointMapRequest;
    type Response = GetPointMapResponse;

    fn request_type_name(&self) -> &str { "GetPointMapRequest" }
    fn response_type_name(&self) -> &str { "GetPointMapResponse" }
}
