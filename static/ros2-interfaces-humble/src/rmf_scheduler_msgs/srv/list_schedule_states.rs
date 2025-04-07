use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListScheduleStatesRequest {
    pub modified_after: i64,
}

impl Default for ListScheduleStatesRequest {
    fn default() -> Self {
        ListScheduleStatesRequest {
            modified_after: 0,
        }
    }
}

impl ros2_client::Message for ListScheduleStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListScheduleStatesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::ScheduleState>,
}

impl Default for ListScheduleStatesResponse {
    fn default() -> Self {
        ListScheduleStatesResponse {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListScheduleStatesResponse {}


pub struct ListScheduleStates;
impl ros2_client::Service for ListScheduleStates {
    type Request = ListScheduleStatesRequest;
    type Response = ListScheduleStatesResponse;

    fn request_type_name(&self) -> &str { "ListScheduleStatesRequest" }
    fn response_type_name(&self) -> &str { "ListScheduleStatesResponse" }
}
