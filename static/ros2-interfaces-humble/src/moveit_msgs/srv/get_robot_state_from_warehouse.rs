use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for GetRobotStateFromWarehouseRequest {
    fn default() -> Self {
        GetRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRobotStateFromWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseResponse {
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetRobotStateFromWarehouseResponse {
    fn default() -> Self {
        GetRobotStateFromWarehouseResponse {
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

impl ros2_client::Message for GetRobotStateFromWarehouseResponse {}


pub struct GetRobotStateFromWarehouse;
impl ros2_client::Service for GetRobotStateFromWarehouse {
    type Request = GetRobotStateFromWarehouseRequest;
    type Response = GetRobotStateFromWarehouseResponse;

    fn request_type_name(&self) -> &str { "GetRobotStateFromWarehouseRequest" }
    fn response_type_name(&self) -> &str { "GetRobotStateFromWarehouseResponse" }
}
