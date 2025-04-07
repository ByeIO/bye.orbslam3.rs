use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidEPH {
    pub svid: u32,
    pub how: u32,
    pub sf1d: Vec<u32>,
    pub sf2d: Vec<u32>,
    pub sf3d: Vec<u32>,
}

impl AidEPH {
    pub const CLASS_ID: u8 = 11;
    pub const MESSAGE_ID: u8 = 49;
}

impl Default for AidEPH {
    fn default() -> Self {
        AidEPH {
            svid: 0,
            how: 0,
            sf1d: Vec::new(),
            sf2d: Vec::new(),
            sf3d: Vec::new(),
        }
    }
}

impl ros2_client::Message for AidEPH {}
