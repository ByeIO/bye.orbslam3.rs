use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleRequest {
    pub schedule: crate::rmf_scheduler_msgs::msg::Schedule,
}

impl Default for CreateScheduleRequest {
    fn default() -> Self {
        CreateScheduleRequest {
            schedule: crate::rmf_scheduler_msgs::msg::Schedule::default(),
        }
    }
}

impl ros2_client::Message for CreateScheduleRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateScheduleResponse {
    fn default() -> Self {
        CreateScheduleResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateScheduleResponse {}


pub struct CreateSchedule;
impl ros2_client::Service for CreateSchedule {
    type Request = CreateScheduleRequest;
    type Response = CreateScheduleResponse;

    fn request_type_name(&self) -> &str { "CreateScheduleRequest" }
    fn response_type_name(&self) -> &str { "CreateScheduleResponse" }
}
