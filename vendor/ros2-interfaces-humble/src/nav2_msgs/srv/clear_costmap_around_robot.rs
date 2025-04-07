use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotRequest {
    pub reset_distance: f32,
}

impl Default for ClearCostmapAroundRobotRequest {
    fn default() -> Self {
        ClearCostmapAroundRobotRequest {
            reset_distance: 0.0,
        }
    }
}

impl ros2_client::Message for ClearCostmapAroundRobotRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapAroundRobotResponse {
    fn default() -> Self {
        ClearCostmapAroundRobotResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearCostmapAroundRobotResponse {}


pub struct ClearCostmapAroundRobot;
impl ros2_client::Service for ClearCostmapAroundRobot {
    type Request = ClearCostmapAroundRobotRequest;
    type Response = ClearCostmapAroundRobotResponse;

    fn request_type_name(&self) -> &str { "ClearCostmapAroundRobotRequest" }
    fn response_type_name(&self) -> &str { "ClearCostmapAroundRobotResponse" }
}
