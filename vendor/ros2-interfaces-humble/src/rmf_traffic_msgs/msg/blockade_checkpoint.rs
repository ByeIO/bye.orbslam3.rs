use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockadeCheckpoint {
    pub position: [f64; 2],
    pub map_name: ::std::string::String,
    pub can_hold: bool,
}

impl Default for BlockadeCheckpoint {
    fn default() -> Self {
        BlockadeCheckpoint {
            position: [0.0; 2],
            map_name: ::std::string::String::new(),
            can_hold: false,
        }
    }
}

impl ros2_client::Message for BlockadeCheckpoint {}
