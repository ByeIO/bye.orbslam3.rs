use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryControllerStatesRequest {
    pub updates: Vec<crate::robot_controllers_msgs::msg::ControllerState>,
}

impl Default for QueryControllerStatesRequest {
    fn default() -> Self {
        QueryControllerStatesRequest {
            updates: Vec::new(),
        }
    }
}

impl ros2_client::Message for QueryControllerStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryControllerStatesResponse {
    pub state: Vec<crate::robot_controllers_msgs::msg::ControllerState>,
}

impl Default for QueryControllerStatesResponse {
    fn default() -> Self {
        QueryControllerStatesResponse {
            state: Vec::new(),
        }
    }
}

impl ros2_client::Message for QueryControllerStatesResponse {}


pub struct QueryControllerStates;
impl ros2_client::Service for QueryControllerStates {
    type Request = QueryControllerStatesRequest;
    type Response = QueryControllerStatesResponse;

    fn request_type_name(&self) -> &str { "QueryControllerStatesRequest" }
    fn response_type_name(&self) -> &str { "QueryControllerStatesResponse" }
}
