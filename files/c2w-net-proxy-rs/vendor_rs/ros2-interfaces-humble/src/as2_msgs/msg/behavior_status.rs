use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorStatus {
    pub status: u8,
}

impl BehaviorStatus {
    pub const IDLE: u8 = 0;
    pub const RUNNING: u8 = 1;
    pub const PAUSED: u8 = 2;
}

impl Default for BehaviorStatus {
    fn default() -> Self {
        BehaviorStatus {
            status: 0,
        }
    }
}

impl ros2_client::Message for BehaviorStatus {}
