use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaySong {
    pub song: u8,
}

impl Default for PlaySong {
    fn default() -> Self {
        PlaySong {
            song: 0,
        }
    }
}

impl ros2_client::Message for PlaySong {}
