use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdSOSAck {
    pub cmd: u8,
    pub reserved0: [u8; 3],
    pub response: u8,
    pub reserved1: [u8; 3],
}

impl UpdSOSAck {
    pub const CLASS_ID: u8 = 9;
    pub const MESSAGE_ID: u8 = 20;
    pub const CMD_BACKUP_CREATE_ACK: u8 = 2;
    pub const CMD_SYSTEM_RESTORED: u8 = 3;
    pub const BACKUP_CREATE_NACK: u8 = 0;
    pub const BACKUP_CREATE_ACK: u8 = 1;
    pub const SYSTEM_RESTORED_RESPONSE_UNKNOWN: u8 = 0;
    pub const SYSTEM_RESTORED_RESPONSE_FAILED: u8 = 1;
    pub const SYSTEM_RESTORED_RESPONSE_RESTORED: u8 = 2;
    pub const SYSTEM_RESTORED_RESPONSE_NOT_RESTORED: u8 = 3;
}

impl Default for UpdSOSAck {
    fn default() -> Self {
        UpdSOSAck {
            cmd: 0,
            reserved0: [0; 3],
            response: 0,
            reserved1: [0; 3],
        }
    }
}

impl ros2_client::Message for UpdSOSAck {}
