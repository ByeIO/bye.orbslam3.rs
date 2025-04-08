use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapExceptRegionRequest {
    pub reset_distance: f32,
}

impl Default for ClearCostmapExceptRegionRequest {
    fn default() -> Self {
        ClearCostmapExceptRegionRequest {
            reset_distance: 0.0,
        }
    }
}

impl ros2_client::Message for ClearCostmapExceptRegionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapExceptRegionResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapExceptRegionResponse {
    fn default() -> Self {
        ClearCostmapExceptRegionResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearCostmapExceptRegionResponse {}


pub struct ClearCostmapExceptRegion;
impl ros2_client::Service for ClearCostmapExceptRegion {
    type Request = ClearCostmapExceptRegionRequest;
    type Response = ClearCostmapExceptRegionResponse;

    fn request_type_name(&self) -> &str { "ClearCostmapExceptRegionRequest" }
    fn response_type_name(&self) -> &str { "ClearCostmapExceptRegionResponse" }
}
