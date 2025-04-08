use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavPOSLLH {
    pub i_tow: u32,
    pub lon: i32,
    pub lat: i32,
    pub height: i32,
    pub h_msl: i32,
    pub h_acc: u32,
    pub v_acc: u32,
}

impl NavPOSLLH {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 2;
}

impl Default for NavPOSLLH {
    fn default() -> Self {
        NavPOSLLH {
            i_tow: 0,
            lon: 0,
            lat: 0,
            height: 0,
            h_msl: 0,
            h_acc: 0,
            v_acc: 0,
        }
    }
}

impl ros2_client::Message for NavPOSLLH {}
