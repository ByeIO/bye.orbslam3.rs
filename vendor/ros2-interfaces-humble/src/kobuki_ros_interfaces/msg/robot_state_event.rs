use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotStateEvent {
    pub state: u8,
}

impl RobotStateEvent {
    pub const ONLINE: u8 = 1;
    pub const OFFLINE: u8 = 0;
}

impl Default for RobotStateEvent {
    fn default() -> Self {
        RobotStateEvent {
            state: 0,
        }
    }
}

impl ros2_client::Message for RobotStateEvent {}
