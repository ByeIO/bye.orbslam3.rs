use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorStatusInfo {
    pub protocol: u8,
    pub err_status: u8,
    pub msg_used: u8,
    pub correction_id: u16,
    pub msg_type_valid: bool,
    pub msg_sub_type_valid: bool,
    pub msg_input_handle: bool,
    pub msg_encrypted: u8,
    pub msg_decrypted: u8,
}

impl CorStatusInfo {
    pub const PROTOCOL_UNKNOWN: u8 = 0;
    pub const PROTOCOL_RTCM3: u8 = 1;
    pub const PROTOCOL_SPARTN: u8 = 2;
    pub const PROTOCOL_UBX_RXM_PMP: u8 = 29;
    pub const PROTOCOL_UBX_RXM_QZSSL6: u8 = 30;
    pub const ERR_UNKNOWN: u8 = 0;
    pub const ERR_ERROR_FREE: u8 = 1;
    pub const ERR_ERRONEOUS: u8 = 2;
    pub const MSG_USED_UNKNOWN: u8 = 0;
    pub const MSG_NOT_USED: u8 = 1;
    pub const MSG_USED: u8 = 2;
    pub const MSG_ENCRYPTION_UNKNOWN: u8 = 0;
    pub const MSG_NOT_ENCRYPTED: u8 = 1;
    pub const MSG_ENCRYPTED: u8 = 2;
    pub const MSG_DECRYPTION_UNKNOWN: u8 = 0;
    pub const MSG_NOT_DECRYPTED: u8 = 1;
    pub const MSG_DECRYPTED: u8 = 2;
}

impl Default for CorStatusInfo {
    fn default() -> Self {
        CorStatusInfo {
            protocol: 0,
            err_status: 0,
            msg_used: 0,
            correction_id: 0,
            msg_type_valid: false,
            msg_sub_type_valid: false,
            msg_input_handle: false,
            msg_encrypted: 0,
            msg_decrypted: 0,
        }
    }
}

impl ros2_client::Message for CorStatusInfo {}
