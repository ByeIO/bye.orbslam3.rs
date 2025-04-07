use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostmapRequest {
    pub specs: crate::nav2_msgs::msg::CostmapMetaData,
}

impl Default for GetCostmapRequest {
    fn default() -> Self {
        GetCostmapRequest {
            specs: crate::nav2_msgs::msg::CostmapMetaData::default(),
        }
    }
}

impl ros2_client::Message for GetCostmapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostmapResponse {
    pub map: crate::nav2_msgs::msg::Costmap,
}

impl Default for GetCostmapResponse {
    fn default() -> Self {
        GetCostmapResponse {
            map: crate::nav2_msgs::msg::Costmap::default(),
        }
    }
}

impl ros2_client::Message for GetCostmapResponse {}


pub struct GetCostmap;
impl ros2_client::Service for GetCostmap {
    type Request = GetCostmapRequest;
    type Response = GetCostmapResponse;

    fn request_type_name(&self) -> &str { "GetCostmapRequest" }
    fn response_type_name(&self) -> &str { "GetCostmapResponse" }
}
