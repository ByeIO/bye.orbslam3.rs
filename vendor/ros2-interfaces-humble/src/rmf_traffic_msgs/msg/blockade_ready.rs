use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockadeReady {
    pub participant: u64,
    pub reservation: u64,
    pub checkpoint: u64,
}

impl Default for BlockadeReady {
    fn default() -> Self {
        BlockadeReady {
            participant: 0,
            reservation: 0,
            checkpoint: 0,
        }
    }
}

impl ros2_client::Message for BlockadeReady {}
