use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointmapLayerRequest {
    pub layer_name: ::std::string::String,
}

impl Default for GetPointmapLayerRequest {
    fn default() -> Self {
        GetPointmapLayerRequest {
            layer_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPointmapLayerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointmapLayerResponse {
    pub valid: bool,
    pub points: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointmapLayerResponse {
    fn default() -> Self {
        GetPointmapLayerResponse {
            valid: false,
            points: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for GetPointmapLayerResponse {}


pub struct GetPointmapLayer;
impl ros2_client::Service for GetPointmapLayer {
    type Request = GetPointmapLayerRequest;
    type Response = GetPointmapLayerResponse;

    fn request_type_name(&self) -> &str { "GetPointmapLayerRequest" }
    fn response_type_name(&self) -> &str { "GetPointmapLayerResponse" }
}
