use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for CheckIfRobotStateExistsInWarehouseRequest {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseResponse {
    pub exists: bool,
}

impl Default for CheckIfRobotStateExistsInWarehouseResponse {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseResponse {
            exists: false,
        }
    }
}

impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseResponse {}


pub struct CheckIfRobotStateExistsInWarehouse;
impl ros2_client::Service for CheckIfRobotStateExistsInWarehouse {
    type Request = CheckIfRobotStateExistsInWarehouseRequest;
    type Response = CheckIfRobotStateExistsInWarehouseResponse;

    fn request_type_name(&self) -> &str { "CheckIfRobotStateExistsInWarehouseRequest" }
    fn response_type_name(&self) -> &str { "CheckIfRobotStateExistsInWarehouseResponse" }
}
