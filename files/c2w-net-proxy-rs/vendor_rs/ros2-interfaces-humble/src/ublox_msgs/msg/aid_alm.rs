use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidALM {
    pub svid: u32,
    pub week: u32,
    pub dwrd: Vec<u32>,
}

impl AidALM {
    pub const CLASS_ID: u8 = 11;
    pub const MESSAGE_ID: u8 = 48;
}

impl Default for AidALM {
    fn default() -> Self {
        AidALM {
            svid: 0,
            week: 0,
            dwrd: Vec::new(),
        }
    }
}

impl ros2_client::Message for AidALM {}
