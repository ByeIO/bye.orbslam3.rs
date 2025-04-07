use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeopathToPathRequest {
    pub geo_path: crate::geographic_msgs::msg::GeoPath,
}

impl Default for GeopathToPathRequest {
    fn default() -> Self {
        GeopathToPathRequest {
            geo_path: crate::geographic_msgs::msg::GeoPath::default(),
        }
    }
}

impl ros2_client::Message for GeopathToPathRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeopathToPathResponse {
    pub success: bool,
    pub path: crate::nav_msgs::msg::Path,
}

impl Default for GeopathToPathResponse {
    fn default() -> Self {
        GeopathToPathResponse {
            success: false,
            path: crate::nav_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for GeopathToPathResponse {}


pub struct GeopathToPath;
impl ros2_client::Service for GeopathToPath {
    type Request = GeopathToPathRequest;
    type Response = GeopathToPathResponse;

    fn request_type_name(&self) -> &str { "GeopathToPathRequest" }
    fn response_type_name(&self) -> &str { "GeopathToPathResponse" }
}
