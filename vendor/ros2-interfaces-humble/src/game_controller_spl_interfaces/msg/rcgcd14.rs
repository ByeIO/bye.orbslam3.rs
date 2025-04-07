use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RCGCD14 {
    pub packet_number: u8,
    pub players_per_team: u8,
    pub competition_phase: u8,
    pub competition_type: u8,
    pub game_phase: u8,
    pub state: u8,
    pub set_play: u8,
    pub first_half: u8,
    pub kicking_team: u8,
    pub secs_remaining: i16,
    pub secondary_time: i16,
    pub teams: [crate::game_controller_spl_interfaces::msg::TeamInfo14; 2],
}

impl RCGCD14 {
    pub const COMPETITION_PHASE_ROUNDROBIN: u8 = 0;
    pub const COMPETITION_PHASE_PLAYOFF: u8 = 1;
    pub const COMPETITION_TYPE_NORMAL: u8 = 0;
    pub const COMPETITION_TYPE_CHALLENGE_SHIELD: u8 = 1;
    pub const COMPETITION_TYPE_7V7: u8 = 2;
    pub const COMPETITION_TYPE_DYNAMIC_BALL_HANDLING: u8 = 3;
    pub const GAME_PHASE_NORMAL: u8 = 0;
    pub const GAME_PHASE_PENALTYSHOOT: u8 = 1;
    pub const GAME_PHASE_OVERTIME: u8 = 2;
    pub const GAME_PHASE_TIMEOUT: u8 = 3;
    pub const STATE_INITIAL: u8 = 0;
    pub const STATE_READY: u8 = 1;
    pub const STATE_SET: u8 = 2;
    pub const STATE_PLAYING: u8 = 3;
    pub const STATE_FINISHED: u8 = 4;
    pub const SET_PLAY_NONE: u8 = 0;
    pub const SET_PLAY_GOAL_KICK: u8 = 1;
    pub const SET_PLAY_PUSHING_FREE_KICK: u8 = 2;
    pub const SET_PLAY_CORNER_KICK: u8 = 3;
    pub const SET_PLAY_KICK_IN: u8 = 4;
    pub const SET_PLAY_PENALTY_KICK: u8 = 5;
}

impl Default for RCGCD14 {
    fn default() -> Self {
        RCGCD14 {
            packet_number: 0,
            players_per_team: 0,
            competition_phase: 0,
            competition_type: 0,
            game_phase: 0,
            state: 0,
            set_play: 0,
            first_half: 0,
            kicking_team: 0,
            secs_remaining: 0,
            secondary_time: 0,
            teams: core::array::from_fn(|_| crate::game_controller_spl_interfaces::msg::TeamInfo14::default()),
        }
    }
}

impl ros2_client::Message for RCGCD14 {}
