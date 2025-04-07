use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerInfo {
    #[serde(rename = "type")]    pub type_: u8,
    pub p_gain: f64,
    pub i_gain: f64,
    pub d_gain: f64,
}

impl ControllerInfo {
    pub const DEFAULT: u8 = 0;
    pub const USER_CONFIGURED: u8 = 1;
}

impl Default for ControllerInfo {
    fn default() -> Self {
        ControllerInfo {
            type_: 0,
            p_gain: 0.0,
            i_gain: 0.0,
            d_gain: 0.0,
        }
    }
}

impl ros2_client::Message for ControllerInfo {}
