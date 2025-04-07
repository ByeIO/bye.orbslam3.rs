use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioStatus {
    pub header: crate::std_msgs::msg::Header,
    pub rssi: u8,
    pub remrssi: u8,
    pub txbuf: u8,
    pub noise: u8,
    pub remnoise: u8,
    pub rxerrors: u16,
    pub fixed: u16,
    pub rssi_dbm: f32,
    pub remrssi_dbm: f32,
}

impl Default for RadioStatus {
    fn default() -> Self {
        RadioStatus {
            header: crate::std_msgs::msg::Header::default(),
            rssi: 0,
            remrssi: 0,
            txbuf: 0,
            noise: 0,
            remnoise: 0,
            rxerrors: 0,
            fixed: 0,
            rssi_dbm: 0.0,
            remrssi_dbm: 0.0,
        }
    }
}

impl ros2_client::Message for RadioStatus {}
