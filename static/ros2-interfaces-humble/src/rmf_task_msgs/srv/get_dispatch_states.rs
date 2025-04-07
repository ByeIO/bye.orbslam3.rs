use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesRequest {
    pub task_ids: Vec<::std::string::String>,
}

impl Default for GetDispatchStatesRequest {
    fn default() -> Self {
        GetDispatchStatesRequest {
            task_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDispatchStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesResponse {
    pub success: bool,
    pub states: crate::rmf_task_msgs::msg::DispatchStates,
}

impl Default for GetDispatchStatesResponse {
    fn default() -> Self {
        GetDispatchStatesResponse {
            success: false,
            states: crate::rmf_task_msgs::msg::DispatchStates::default(),
        }
    }
}

impl ros2_client::Message for GetDispatchStatesResponse {}


pub struct GetDispatchStates;
impl ros2_client::Service for GetDispatchStates {
    type Request = GetDispatchStatesRequest;
    type Response = GetDispatchStatesResponse;

    fn request_type_name(&self) -> &str { "GetDispatchStatesRequest" }
    fn response_type_name(&self) -> &str { "GetDispatchStatesResponse" }
}
