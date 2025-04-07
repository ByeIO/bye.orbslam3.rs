use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftClearanceRequest {
    pub robot_name: ::std::string::String,
    pub lift_name: ::std::string::String,
}

impl Default for LiftClearanceRequest {
    fn default() -> Self {
        LiftClearanceRequest {
            robot_name: ::std::string::String::new(),
            lift_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LiftClearanceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftClearanceResponse {
    pub decision: u32,
}

impl LiftClearanceResponse {
    pub const DECISION_CLEAR: u32 = 1;
    pub const DECISION_CROWDED: u32 = 2;
}

impl Default for LiftClearanceResponse {
    fn default() -> Self {
        LiftClearanceResponse {
            decision: 0,
        }
    }
}

impl ros2_client::Message for LiftClearanceResponse {}


pub struct LiftClearance;
impl ros2_client::Service for LiftClearance {
    type Request = LiftClearanceRequest;
    type Response = LiftClearanceResponse;

    fn request_type_name(&self) -> &str { "LiftClearanceRequest" }
    fn response_type_name(&self) -> &str { "LiftClearanceResponse" }
}
