use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mavlink {
    pub header: crate::std_msgs::msg::Header,
    pub framing_status: u8,
    pub magic: u8,
    pub len: u8,
    pub incompat_flags: u8,
    pub compat_flags: u8,
    pub seq: u8,
    pub sysid: u8,
    pub compid: u8,
    pub msgid: u32,
    pub checksum: u16,
    pub payload64: Vec<u64>,
    pub signature: Vec<u8>,
}

impl Mavlink {
    pub const FRAMING_OK: u8 = 1;
    pub const FRAMING_BAD_CRC: u8 = 2;
    pub const FRAMING_BAD_SIGNATURE: u8 = 3;
    pub const MAVLINK_V10: u8 = 254;
    pub const MAVLINK_V20: u8 = 253;
}

impl Default for Mavlink {
    fn default() -> Self {
        Mavlink {
            header: crate::std_msgs::msg::Header::default(),
            framing_status: 0,
            magic: 0,
            len: 0,
            incompat_flags: 0,
            compat_flags: 0,
            seq: 0,
            sysid: 0,
            compid: 0,
            msgid: 0,
            checksum: 0,
            payload64: Vec::new(),
            signature: Vec::new(),
        }
    }
}

impl ros2_client::Message for Mavlink {}
