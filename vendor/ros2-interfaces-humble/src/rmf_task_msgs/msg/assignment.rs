use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub is_assigned: bool,
    pub fleet_name: ::std::string::String,
    pub expected_robot_name: ::std::string::String,
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment {
            is_assigned: false,
            fleet_name: ::std::string::String::new(),
            expected_robot_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Assignment {}
