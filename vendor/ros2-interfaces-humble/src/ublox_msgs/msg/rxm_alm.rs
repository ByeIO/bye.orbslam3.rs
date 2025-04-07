use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmALM {
    pub svid: u32,
    pub week: u32,
    pub dwrd: Vec<u32>,
}

impl RxmALM {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 48;
}

impl Default for RxmALM {
    fn default() -> Self {
        RxmALM {
            svid: 0,
            week: 0,
            dwrd: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmALM {}
