use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapRequest {
    fn default() -> Self {
        ClearEntireCostmapRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearEntireCostmapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapResponse {
    fn default() -> Self {
        ClearEntireCostmapResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearEntireCostmapResponse {}


pub struct ClearEntireCostmap;
impl ros2_client::Service for ClearEntireCostmap {
    type Request = ClearEntireCostmapRequest;
    type Response = ClearEntireCostmapResponse;

    fn request_type_name(&self) -> &str { "ClearEntireCostmapRequest" }
    fn response_type_name(&self) -> &str { "ClearEntireCostmapResponse" }
}
