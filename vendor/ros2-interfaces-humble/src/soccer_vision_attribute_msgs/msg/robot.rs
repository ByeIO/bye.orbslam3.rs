use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Robot {
    pub player_number: u8,
    pub team: u8,
    pub state: u8,
    pub facing: u8,
}

impl Robot {
    pub const NUMBER_UNKNOWN: u8 = 0;
    pub const TEAM_UNKNOWN: u8 = 0;
    pub const TEAM_OWN: u8 = 1;
    pub const TEAM_OPPONENT: u8 = 2;
    pub const STATE_UNKNOWN: u8 = 0;
    pub const STATE_STANDING: u8 = 1;
    pub const STATE_FALLEN: u8 = 2;
    pub const STATE_KICKING: u8 = 3;
    pub const STATE_INACTIVE: u8 = 4;
    pub const FACING_UNKNOWN: u8 = 0;
    pub const FACING_THIS_WAY: u8 = 1;
    pub const FACING_AWAY: u8 = 2;
    pub const FACING_LEFT: u8 = 3;
    pub const FACING_RIGHT: u8 = 4;
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            player_number: 0,
            team: 0,
            state: 0,
            facing: 0,
        }
    }
}

impl ros2_client::Message for Robot {}
