use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefineSong {
    pub song: u8,
    pub length: u8,
    pub notes: Vec<u8>,
    pub durations: Vec<f32>,
}

impl Default for DefineSong {
    fn default() -> Self {
        DefineSong {
            song: 0,
            length: 0,
            notes: Vec::new(),
            durations: Vec::new(),
        }
    }
}

impl ros2_client::Message for DefineSong {}
