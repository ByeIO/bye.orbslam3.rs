use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmSFRB {
    pub chn: u8,
    pub svid: u8,
    pub dwrd: [u32; 10],
}

impl RxmSFRB {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 17;
}

impl Default for RxmSFRB {
    fn default() -> Self {
        RxmSFRB {
            chn: 0,
            svid: 0,
            dwrd: [0; 10],
        }
    }
}

impl ros2_client::Message for RxmSFRB {}
