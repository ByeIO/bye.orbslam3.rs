use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathToGeopathRequest {
    pub path: crate::nav_msgs::msg::Path,
}

impl Default for PathToGeopathRequest {
    fn default() -> Self {
        PathToGeopathRequest {
            path: crate::nav_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for PathToGeopathRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathToGeopathResponse {
    pub success: bool,
    pub geo_path: crate::geographic_msgs::msg::GeoPath,
}

impl Default for PathToGeopathResponse {
    fn default() -> Self {
        PathToGeopathResponse {
            success: false,
            geo_path: crate::geographic_msgs::msg::GeoPath::default(),
        }
    }
}

impl ros2_client::Message for PathToGeopathResponse {}


pub struct PathToGeopath;
impl ros2_client::Service for PathToGeopath {
    type Request = PathToGeopathRequest;
    type Response = PathToGeopathResponse;

    fn request_type_name(&self) -> &str { "PathToGeopathRequest" }
    fn response_type_name(&self) -> &str { "PathToGeopathResponse" }
}
