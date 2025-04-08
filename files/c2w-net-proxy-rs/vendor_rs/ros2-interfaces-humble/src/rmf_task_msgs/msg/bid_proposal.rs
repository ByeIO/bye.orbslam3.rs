use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidProposal {
    pub fleet_name: ::std::string::String,
    pub expected_robot_name: ::std::string::String,
    pub prev_cost: f64,
    pub new_cost: f64,
    pub finish_time: crate::builtin_interfaces::msg::Time,
}

impl Default for BidProposal {
    fn default() -> Self {
        BidProposal {
            fleet_name: ::std::string::String::new(),
            expected_robot_name: ::std::string::String::new(),
            prev_cost: 0.0,
            new_cost: 0.0,
            finish_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for BidProposal {}
