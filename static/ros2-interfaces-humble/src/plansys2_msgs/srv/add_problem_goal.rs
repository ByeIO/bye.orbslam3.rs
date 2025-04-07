use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemGoalRequest {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for AddProblemGoalRequest {
    fn default() -> Self {
        AddProblemGoalRequest {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

impl ros2_client::Message for AddProblemGoalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemGoalResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemGoalResponse {
    fn default() -> Self {
        AddProblemGoalResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddProblemGoalResponse {}


pub struct AddProblemGoal;
impl ros2_client::Service for AddProblemGoal {
    type Request = AddProblemGoalRequest;
    type Response = AddProblemGoalResponse;

    fn request_type_name(&self) -> &str { "AddProblemGoalRequest" }
    fn response_type_name(&self) -> &str { "AddProblemGoalResponse" }
}
