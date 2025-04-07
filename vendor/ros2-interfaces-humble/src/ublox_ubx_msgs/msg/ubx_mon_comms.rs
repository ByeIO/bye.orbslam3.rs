use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXMonComms {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub n_ports: u8,
    pub tx_errors: u8,
    pub reserved0: u8,
    pub prot_ids: [u8; 4],
    pub ports: Vec<crate::ublox_ubx_msgs::msg::CommsPortInfo>,
}

impl UBXMonComms {
    pub const PROTOCOL_UBX: u8 = 0;
    pub const PROTOCOL_NMEA: u8 = 1;
    pub const PROTOCOL_RTCM2: u8 = 2;
    pub const PROTOCOL_RTCM3: u8 = 5;
    pub const PROTOCOL_SPARTN: u8 = 6;
    pub const NO_PROTOCOL: u8 = 255;
}

impl Default for UBXMonComms {
    fn default() -> Self {
        UBXMonComms {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            n_ports: 0,
            tx_errors: 0,
            reserved0: 0,
            prot_ids: [0; 4],
            ports: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXMonComms {}
