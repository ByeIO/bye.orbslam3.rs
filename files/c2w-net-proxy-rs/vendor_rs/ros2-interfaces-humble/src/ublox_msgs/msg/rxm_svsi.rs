use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmSVSI {
    pub i_tow: i32,
    pub week: i16,
    pub num_vis: u8,
    pub num_sv: u8,
    pub sv: Vec<crate::ublox_msgs::msg::RxmSVSISV>,
}

impl RxmSVSI {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 32;
}

impl Default for RxmSVSI {
    fn default() -> Self {
        RxmSVSI {
            i_tow: 0,
            week: 0,
            num_vis: 0,
            num_sv: 0,
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmSVSI {}
