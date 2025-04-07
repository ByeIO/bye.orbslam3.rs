use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggerStatesRequest {
    pub modified_after: i64,
}

impl Default for ListTriggerStatesRequest {
    fn default() -> Self {
        ListTriggerStatesRequest {
            modified_after: 0,
        }
    }
}

impl ros2_client::Message for ListTriggerStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggerStatesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::TriggerState>,
}

impl Default for ListTriggerStatesResponse {
    fn default() -> Self {
        ListTriggerStatesResponse {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListTriggerStatesResponse {}


pub struct ListTriggerStates;
impl ros2_client::Service for ListTriggerStates {
    type Request = ListTriggerStatesRequest;
    type Response = ListTriggerStatesResponse;

    fn request_type_name(&self) -> &str { "ListTriggerStatesRequest" }
    fn response_type_name(&self) -> &str { "ListTriggerStatesResponse" }
}
