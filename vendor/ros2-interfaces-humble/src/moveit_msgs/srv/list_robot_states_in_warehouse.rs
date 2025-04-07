use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRobotStatesInWarehouseRequest {
    pub regex: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for ListRobotStatesInWarehouseRequest {
    fn default() -> Self {
        ListRobotStatesInWarehouseRequest {
            regex: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListRobotStatesInWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRobotStatesInWarehouseResponse {
    pub states: Vec<::std::string::String>,
}

impl Default for ListRobotStatesInWarehouseResponse {
    fn default() -> Self {
        ListRobotStatesInWarehouseResponse {
            states: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListRobotStatesInWarehouseResponse {}


pub struct ListRobotStatesInWarehouse;
impl ros2_client::Service for ListRobotStatesInWarehouse {
    type Request = ListRobotStatesInWarehouseRequest;
    type Response = ListRobotStatesInWarehouseResponse;

    fn request_type_name(&self) -> &str { "ListRobotStatesInWarehouseRequest" }
    fn response_type_name(&self) -> &str { "ListRobotStatesInWarehouseResponse" }
}
