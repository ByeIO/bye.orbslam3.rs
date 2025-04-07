use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateValidityRequest {
    pub robot_state: crate::moveit_msgs::msg::RobotState,
    pub group_name: ::std::string::String,
    pub constraints: crate::moveit_msgs::msg::Constraints,
}

impl Default for GetStateValidityRequest {
    fn default() -> Self {
        GetStateValidityRequest {
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
            group_name: ::std::string::String::new(),
            constraints: crate::moveit_msgs::msg::Constraints::default(),
        }
    }
}

impl ros2_client::Message for GetStateValidityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateValidityResponse {
    pub valid: bool,
    pub contacts: Vec<crate::moveit_msgs::msg::ContactInformation>,
    pub cost_sources: Vec<crate::moveit_msgs::msg::CostSource>,
    pub constraint_result: Vec<crate::moveit_msgs::msg::ConstraintEvalResult>,
}

impl Default for GetStateValidityResponse {
    fn default() -> Self {
        GetStateValidityResponse {
            valid: false,
            contacts: Vec::new(),
            cost_sources: Vec::new(),
            constraint_result: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetStateValidityResponse {}


pub struct GetStateValidity;
impl ros2_client::Service for GetStateValidity {
    type Request = GetStateValidityRequest;
    type Response = GetStateValidityResponse;

    fn request_type_name(&self) -> &str { "GetStateValidityRequest" }
    fn response_type_name(&self) -> &str { "GetStateValidityResponse" }
}
