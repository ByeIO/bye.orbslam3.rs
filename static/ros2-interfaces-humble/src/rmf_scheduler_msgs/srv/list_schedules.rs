use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesRequest {
    pub created_after: i64,
}

impl Default for ListSchedulesRequest {
    fn default() -> Self {
        ListSchedulesRequest {
            created_after: 0,
        }
    }
}

impl ros2_client::Message for ListSchedulesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::Schedule>,
}

impl Default for ListSchedulesResponse {
    fn default() -> Self {
        ListSchedulesResponse {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListSchedulesResponse {}


pub struct ListSchedules;
impl ros2_client::Service for ListSchedules {
    type Request = ListSchedulesRequest;
    type Response = ListSchedulesResponse;

    fn request_type_name(&self) -> &str { "ListSchedulesRequest" }
    fn response_type_name(&self) -> &str { "ListSchedulesResponse" }
}
