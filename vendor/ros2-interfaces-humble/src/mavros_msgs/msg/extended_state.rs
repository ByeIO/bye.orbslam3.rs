use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedState {
    pub header: crate::std_msgs::msg::Header,
    pub vtol_state: u8,
    pub landed_state: u8,
}

impl ExtendedState {
    pub const VTOL_STATE_UNDEFINED: u8 = 0;
    pub const VTOL_STATE_TRANSITION_TO_FW: u8 = 1;
    pub const VTOL_STATE_TRANSITION_TO_MC: u8 = 2;
    pub const VTOL_STATE_MC: u8 = 3;
    pub const VTOL_STATE_FW: u8 = 4;
    pub const LANDED_STATE_UNDEFINED: u8 = 0;
    pub const LANDED_STATE_ON_GROUND: u8 = 1;
    pub const LANDED_STATE_IN_AIR: u8 = 2;
    pub const LANDED_STATE_TAKEOFF: u8 = 3;
    pub const LANDED_STATE_LANDING: u8 = 4;
}

impl Default for ExtendedState {
    fn default() -> Self {
        ExtendedState {
            header: crate::std_msgs::msg::Header::default(),
            vtol_state: 0,
            landed_state: 0,
        }
    }
}

impl ros2_client::Message for ExtendedState {}
