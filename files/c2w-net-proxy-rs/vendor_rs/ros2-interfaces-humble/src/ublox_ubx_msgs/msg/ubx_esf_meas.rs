use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXEsfMeas {
    pub header: crate::std_msgs::msg::Header,
    pub time_tag: u32,
    pub time_mark_sent: u8,
    pub time_mark_edge: bool,
    pub calib_ttag_valid: bool,
    pub num_meas: u8,
    pub id: u16,
    pub data: Vec<crate::ublox_ubx_msgs::msg::ESFMeasDataItem>,
    pub calib_ttag: u32,
}

impl Default for UBXEsfMeas {
    fn default() -> Self {
        UBXEsfMeas {
            header: crate::std_msgs::msg::Header::default(),
            time_tag: 0,
            time_mark_sent: 0,
            time_mark_edge: false,
            calib_ttag_valid: false,
            num_meas: 0,
            id: 0,
            data: Vec::new(),
            calib_ttag: 0,
        }
    }
}

impl ros2_client::Message for UBXEsfMeas {}
