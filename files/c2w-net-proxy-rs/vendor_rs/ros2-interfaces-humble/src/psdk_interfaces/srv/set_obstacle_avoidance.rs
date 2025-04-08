use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetObstacleAvoidanceRequest {
    pub obstacle_avoidance_on: bool,
}

impl Default for SetObstacleAvoidanceRequest {
    fn default() -> Self {
        SetObstacleAvoidanceRequest {
            obstacle_avoidance_on: false,
        }
    }
}

impl ros2_client::Message for SetObstacleAvoidanceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetObstacleAvoidanceResponse {
    pub success: bool,
}

impl Default for SetObstacleAvoidanceResponse {
    fn default() -> Self {
        SetObstacleAvoidanceResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetObstacleAvoidanceResponse {}


pub struct SetObstacleAvoidance;
impl ros2_client::Service for SetObstacleAvoidance {
    type Request = SetObstacleAvoidanceRequest;
    type Response = SetObstacleAvoidanceResponse;

    fn request_type_name(&self) -> &str { "SetObstacleAvoidanceRequest" }
    fn response_type_name(&self) -> &str { "SetObstacleAvoidanceResponse" }
}
