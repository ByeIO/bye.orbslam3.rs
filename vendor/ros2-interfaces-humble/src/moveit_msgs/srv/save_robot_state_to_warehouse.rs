use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for SaveRobotStateToWarehouseRequest {
    fn default() -> Self {
        SaveRobotStateToWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

impl ros2_client::Message for SaveRobotStateToWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseResponse {
    pub success: bool,
}

impl Default for SaveRobotStateToWarehouseResponse {
    fn default() -> Self {
        SaveRobotStateToWarehouseResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveRobotStateToWarehouseResponse {}


pub struct SaveRobotStateToWarehouse;
impl ros2_client::Service for SaveRobotStateToWarehouse {
    type Request = SaveRobotStateToWarehouseRequest;
    type Response = SaveRobotStateToWarehouseResponse;

    fn request_type_name(&self) -> &str { "SaveRobotStateToWarehouseRequest" }
    fn response_type_name(&self) -> &str { "SaveRobotStateToWarehouseResponse" }
}
