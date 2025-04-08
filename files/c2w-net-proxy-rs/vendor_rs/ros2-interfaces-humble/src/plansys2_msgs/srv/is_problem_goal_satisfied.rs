use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProblemGoalSatisfiedRequest {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for IsProblemGoalSatisfiedRequest {
    fn default() -> Self {
        IsProblemGoalSatisfiedRequest {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

impl ros2_client::Message for IsProblemGoalSatisfiedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProblemGoalSatisfiedResponse {
    pub success: bool,
    pub satisfied: bool,
    pub error_info: ::std::string::String,
}

impl Default for IsProblemGoalSatisfiedResponse {
    fn default() -> Self {
        IsProblemGoalSatisfiedResponse {
            success: false,
            satisfied: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for IsProblemGoalSatisfiedResponse {}


pub struct IsProblemGoalSatisfied;
impl ros2_client::Service for IsProblemGoalSatisfied {
    type Request = IsProblemGoalSatisfiedRequest;
    type Response = IsProblemGoalSatisfiedResponse;

    fn request_type_name(&self) -> &str { "IsProblemGoalSatisfiedRequest" }
    fn response_type_name(&self) -> &str { "IsProblemGoalSatisfiedResponse" }
}
