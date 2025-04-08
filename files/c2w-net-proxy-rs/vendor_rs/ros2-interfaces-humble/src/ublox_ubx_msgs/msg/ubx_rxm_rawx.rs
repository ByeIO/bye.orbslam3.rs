use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXRxmRawx {
    pub header: crate::std_msgs::msg::Header,
    pub rcv_tow: f64,
    pub week: u16,
    pub leap_s: i8,
    pub num_meas: u8,
    pub rec_stat: crate::ublox_ubx_msgs::msg::RecStat,
    pub version: u8,
    pub rawx_data: Vec<crate::ublox_ubx_msgs::msg::RawxData>,
}

impl Default for UBXRxmRawx {
    fn default() -> Self {
        UBXRxmRawx {
            header: crate::std_msgs::msg::Header::default(),
            rcv_tow: 0.0,
            week: 0,
            leap_s: 0,
            num_meas: 0,
            rec_stat: crate::ublox_ubx_msgs::msg::RecStat::default(),
            version: 0,
            rawx_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXRxmRawx {}
