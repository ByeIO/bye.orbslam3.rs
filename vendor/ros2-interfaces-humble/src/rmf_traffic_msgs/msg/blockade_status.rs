use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockadeStatus {
    pub participant: u64,
    pub reservation: u64,
    pub any_ready: bool,
    pub last_ready: u64,
    pub last_reached: u64,
    pub assignment_begin: u64,
    pub assignment_end: u64,
}

impl Default for BlockadeStatus {
    fn default() -> Self {
        BlockadeStatus {
            participant: 0,
            reservation: 0,
            any_ready: false,
            last_ready: 0,
            last_reached: 0,
            assignment_begin: 0,
            assignment_end: 0,
        }
    }
}

impl ros2_client::Message for BlockadeStatus {}
