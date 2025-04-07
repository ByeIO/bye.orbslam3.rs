use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapROIRequest {
    pub x: f64,
    pub y: f64,
    pub l_x: f64,
    pub l_y: f64,
}

impl Default for GetMapROIRequest {
    fn default() -> Self {
        GetMapROIRequest {
            x: 0.0,
            y: 0.0,
            l_x: 0.0,
            l_y: 0.0,
        }
    }
}

impl ros2_client::Message for GetMapROIRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapROIResponse {
    pub sub_map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapROIResponse {
    fn default() -> Self {
        GetMapROIResponse {
            sub_map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetMapROIResponse {}


pub struct GetMapROI;
impl ros2_client::Service for GetMapROI {
    type Request = GetMapROIRequest;
    type Response = GetMapROIResponse;

    fn request_type_name(&self) -> &str { "GetMapROIRequest" }
    fn response_type_name(&self) -> &str { "GetMapROIResponse" }
}
