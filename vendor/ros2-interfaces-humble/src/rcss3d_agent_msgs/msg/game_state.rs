use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub time: f32,
    pub playmode: ::std::string::String,
    pub score_left: i32,
    pub score_right: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            time: 0.0,
            playmode: ::std::string::String::new(),
            score_left: 0,
            score_right: 0,
        }
    }
}

impl ros2_client::Message for GameState {}
