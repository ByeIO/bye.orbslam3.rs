use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerRequest {
    pub trigger: crate::rmf_scheduler_msgs::msg::Trigger,
}

impl Default for CreateTriggerRequest {
    fn default() -> Self {
        CreateTriggerRequest {
            trigger: crate::rmf_scheduler_msgs::msg::Trigger::default(),
        }
    }
}

impl ros2_client::Message for CreateTriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateTriggerResponse {
    fn default() -> Self {
        CreateTriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateTriggerResponse {}


pub struct CreateTrigger;
impl ros2_client::Service for CreateTrigger {
    type Request = CreateTriggerRequest;
    type Response = CreateTriggerResponse;

    fn request_type_name(&self) -> &str { "CreateTriggerRequest" }
    fn response_type_name(&self) -> &str { "CreateTriggerResponse" }
}
