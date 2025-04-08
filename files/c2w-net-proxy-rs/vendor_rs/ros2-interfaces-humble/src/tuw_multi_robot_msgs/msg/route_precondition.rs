use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutePrecondition {
    pub robot_id: ::std::string::String,
    pub current_route_segment: i32,
}

impl Default for RoutePrecondition {
    fn default() -> Self {
        RoutePrecondition {
            robot_id: ::std::string::String::new(),
            current_route_segment: 0,
        }
    }
}

impl ros2_client::Message for RoutePrecondition {}
