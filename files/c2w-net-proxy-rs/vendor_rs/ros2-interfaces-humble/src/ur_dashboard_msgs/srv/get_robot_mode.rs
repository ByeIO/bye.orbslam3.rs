use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotModeRequest {

}

impl Default for GetRobotModeRequest {
    fn default() -> Self {
        GetRobotModeRequest {

        }
    }
}

impl ros2_client::Message for GetRobotModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotModeResponse {
    pub robot_mode: crate::ur_dashboard_msgs::msg::RobotMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetRobotModeResponse {
    fn default() -> Self {
        GetRobotModeResponse {
            robot_mode: crate::ur_dashboard_msgs::msg::RobotMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetRobotModeResponse {}


pub struct GetRobotMode;
impl ros2_client::Service for GetRobotMode {
    type Request = GetRobotModeRequest;
    type Response = GetRobotModeResponse;

    fn request_type_name(&self) -> &str { "GetRobotModeRequest" }
    fn response_type_name(&self) -> &str { "GetRobotModeResponse" }
}
