use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tunnel {
    pub target_system: u8,
    pub target_component: u8,
    pub payload_type: u16,
    pub payload_length: u8,
    #[serde_as(as = "[_; 128]")]
    pub payload: [u8; 128],
}

impl Tunnel {
    pub const PAYLOAD_TYPE_UNKNOWN: u16 = 0;
    pub const PAYLOAD_TYPE_STORM32_RESERVED0: u16 = 200;
    pub const PAYLOAD_TYPE_STORM32_RESERVED1: u16 = 201;
    pub const PAYLOAD_TYPE_STORM32_RESERVED2: u16 = 202;
    pub const PAYLOAD_TYPE_STORM32_RESERVED3: u16 = 203;
    pub const PAYLOAD_TYPE_STORM32_RESERVED4: u16 = 204;
    pub const PAYLOAD_TYPE_STORM32_RESERVED5: u16 = 205;
    pub const PAYLOAD_TYPE_STORM32_RESERVED6: u16 = 206;
    pub const PAYLOAD_TYPE_STORM32_RESERVED7: u16 = 207;
    pub const PAYLOAD_TYPE_STORM32_RESERVED8: u16 = 208;
    pub const PAYLOAD_TYPE_STORM32_RESERVED9: u16 = 209;
}

impl Default for Tunnel {
    fn default() -> Self {
        Tunnel {
            target_system: 0,
            target_component: 0,
            payload_type: 0,
            payload_length: 0,
            payload: [0; 128],
        }
    }
}

impl ros2_client::Message for Tunnel {}
