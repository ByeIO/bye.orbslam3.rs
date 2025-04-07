use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrderedSubGoalsRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetOrderedSubGoalsRequest {
    fn default() -> Self {
        GetOrderedSubGoalsRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetOrderedSubGoalsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrderedSubGoalsResponse {
    pub success: bool,
    pub sub_goals: Vec<crate::plansys2_msgs::msg::Tree>,
    pub error_info: ::std::string::String,
}

impl Default for GetOrderedSubGoalsResponse {
    fn default() -> Self {
        GetOrderedSubGoalsResponse {
            success: false,
            sub_goals: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetOrderedSubGoalsResponse {}


pub struct GetOrderedSubGoals;
impl ros2_client::Service for GetOrderedSubGoals {
    type Request = GetOrderedSubGoalsRequest;
    type Response = GetOrderedSubGoalsResponse;

    fn request_type_name(&self) -> &str { "GetOrderedSubGoalsRequest" }
    fn response_type_name(&self) -> &str { "GetOrderedSubGoalsResponse" }
}
