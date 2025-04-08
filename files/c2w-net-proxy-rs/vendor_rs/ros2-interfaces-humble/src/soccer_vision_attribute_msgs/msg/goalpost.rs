use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goalpost {
    pub side: u8,
    pub team: u8,
}

impl Goalpost {
    pub const SIDE_UNKNOWN: u8 = 0;
    pub const SIDE_LEFT: u8 = 1;
    pub const SIDE_RIGHT: u8 = 2;
    pub const TEAM_UNKNOWN: u8 = 0;
    pub const TEAM_OWN: u8 = 1;
    pub const TEAM_OPPONENT: u8 = 2;
}

impl Default for Goalpost {
    fn default() -> Self {
        Goalpost {
            side: 0,
            team: 0,
        }
    }
}

impl ros2_client::Message for Goalpost {}
