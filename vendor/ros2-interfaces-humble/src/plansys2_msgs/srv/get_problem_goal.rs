use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemGoalRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemGoalRequest {
    fn default() -> Self {
        GetProblemGoalRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemGoalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemGoalResponse {
    pub success: bool,
    pub tree: crate::plansys2_msgs::msg::Tree,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemGoalResponse {
    fn default() -> Self {
        GetProblemGoalResponse {
            success: false,
            tree: crate::plansys2_msgs::msg::Tree::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemGoalResponse {}


pub struct GetProblemGoal;
impl ros2_client::Service for GetProblemGoal {
    type Request = GetProblemGoalRequest;
    type Response = GetProblemGoalResponse;

    fn request_type_name(&self) -> &str { "GetProblemGoalRequest" }
    fn response_type_name(&self) -> &str { "GetProblemGoalResponse" }
}
