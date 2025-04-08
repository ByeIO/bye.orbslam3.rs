use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamInfo15 {
    pub team_number: u8,
    pub field_player_colour: u8,
    pub goalkeeper_colour: u8,
    pub goalkeeper: u8,
    pub team_colour: u8,
    pub score: u8,
    pub penalty_shot: u8,
    pub single_shots: u16,
    pub message_budget: u16,
    pub players: [crate::game_controller_spl_interfaces::msg::RobotInfo15; 20],
}

impl TeamInfo15 {
    pub const TEAM_BLUE: u8 = 0;
    pub const TEAM_RED: u8 = 1;
    pub const TEAM_YELLOW: u8 = 2;
    pub const TEAM_BLACK: u8 = 3;
    pub const TEAM_WHITE: u8 = 4;
    pub const TEAM_GREEN: u8 = 5;
    pub const TEAM_ORANGE: u8 = 6;
    pub const TEAM_PURPLE: u8 = 7;
    pub const TEAM_BROWN: u8 = 8;
    pub const TEAM_GRAY: u8 = 9;
}

impl Default for TeamInfo15 {
    fn default() -> Self {
        TeamInfo15 {
            team_number: 0,
            field_player_colour: 0,
            goalkeeper_colour: 0,
            goalkeeper: 0,
            team_colour: 0,
            score: 0,
            penalty_shot: 0,
            single_shots: 0,
            message_budget: 0,
            players: core::array::from_fn(|_| crate::game_controller_spl_interfaces::msg::RobotInfo15::default()),
        }
    }
}

impl ros2_client::Message for TeamInfo15 {}
