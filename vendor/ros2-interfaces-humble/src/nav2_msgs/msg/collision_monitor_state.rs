use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionMonitorState {
    pub action_type: u8,
    pub polygon_name: ::std::string::String,
}

impl CollisionMonitorState {
    pub const DO_NOTHING: u8 = 0;
    pub const STOP: u8 = 1;
    pub const SLOWDOWN: u8 = 2;
    pub const APPROACH: u8 = 3;
}

impl Default for CollisionMonitorState {
    fn default() -> Self {
        CollisionMonitorState {
            action_type: 0,
            polygon_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CollisionMonitorState {}
