use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavPOSECEF {
    pub i_tow: u32,
    pub ecef_x: i32,
    pub ecef_y: i32,
    pub ecef_z: i32,
    pub p_acc: u32,
}

impl NavPOSECEF {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 1;
}

impl Default for NavPOSECEF {
    fn default() -> Self {
        NavPOSECEF {
            i_tow: 0,
            ecef_x: 0,
            ecef_y: 0,
            ecef_z: 0,
            p_acc: 0,
        }
    }
}

impl ros2_client::Message for NavPOSECEF {}
