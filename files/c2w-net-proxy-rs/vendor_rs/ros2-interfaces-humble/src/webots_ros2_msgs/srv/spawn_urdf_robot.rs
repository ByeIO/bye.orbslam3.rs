use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotRequest {
    pub robot: crate::webots_ros2_msgs::msg::UrdfRobot,
}

impl Default for SpawnUrdfRobotRequest {
    fn default() -> Self {
        SpawnUrdfRobotRequest {
            robot: crate::webots_ros2_msgs::msg::UrdfRobot::default(),
        }
    }
}

impl ros2_client::Message for SpawnUrdfRobotRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotResponse {
    pub success: bool,
}

impl Default for SpawnUrdfRobotResponse {
    fn default() -> Self {
        SpawnUrdfRobotResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SpawnUrdfRobotResponse {}


pub struct SpawnUrdfRobot;
impl ros2_client::Service for SpawnUrdfRobot {
    type Request = SpawnUrdfRobotRequest;
    type Response = SpawnUrdfRobotResponse;

    fn request_type_name(&self) -> &str { "SpawnUrdfRobotRequest" }
    fn response_type_name(&self) -> &str { "SpawnUrdfRobotResponse" }
}
