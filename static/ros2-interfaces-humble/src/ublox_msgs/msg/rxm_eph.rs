use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmEPH {
    pub svid: u32,
    pub how: u32,
    pub sf1d: Vec<u32>,
    pub sf2d: Vec<u32>,
    pub sf3d: Vec<u32>,
}

impl RxmEPH {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 49;
}

impl Default for RxmEPH {
    fn default() -> Self {
        RxmEPH {
            svid: 0,
            how: 0,
            sf1d: Vec::new(),
            sf2d: Vec::new(),
            sf3d: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmEPH {}
