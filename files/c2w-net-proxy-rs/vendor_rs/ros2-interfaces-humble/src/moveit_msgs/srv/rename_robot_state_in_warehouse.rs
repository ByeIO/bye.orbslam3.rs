use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameRobotStateInWarehouseRequest {
    pub old_name: ::std::string::String,
    pub new_name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for RenameRobotStateInWarehouseRequest {
    fn default() -> Self {
        RenameRobotStateInWarehouseRequest {
            old_name: ::std::string::String::new(),
            new_name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RenameRobotStateInWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameRobotStateInWarehouseResponse {

}

impl Default for RenameRobotStateInWarehouseResponse {
    fn default() -> Self {
        RenameRobotStateInWarehouseResponse {

        }
    }
}

impl ros2_client::Message for RenameRobotStateInWarehouseResponse {}


pub struct RenameRobotStateInWarehouse;
impl ros2_client::Service for RenameRobotStateInWarehouse {
    type Request = RenameRobotStateInWarehouseRequest;
    type Response = RenameRobotStateInWarehouseResponse;

    fn request_type_name(&self) -> &str { "RenameRobotStateInWarehouseRequest" }
    fn response_type_name(&self) -> &str { "RenameRobotStateInWarehouseResponse" }
}
