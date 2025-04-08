use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SPLSM {
    pub player_num: u8, // default: 0
    pub team_num: u8, // default: 0
    pub fallen: u8, // default: 255
    pub pose: [f32; 3], // default: [0.0, 0.0, 0.0]
    pub ball_age: f32, // default: -1.0
    pub ball: [f32; 2], // default: [0.0, 0.0]
    pub data: Vec<u8>,
}

impl Default for SPLSM {
    fn default() -> Self {
        SPLSM {
            player_num: 0,
            team_num: 0,
            fallen: 255,
            pose: [0.0, 0.0, 0.0],
            ball_age: -1.0,
            ball: [0.0, 0.0],
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for SPLSM {}
