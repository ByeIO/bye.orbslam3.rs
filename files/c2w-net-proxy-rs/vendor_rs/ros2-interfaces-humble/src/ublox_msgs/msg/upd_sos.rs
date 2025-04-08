use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdSOS {
    pub cmd: u8,
    pub reserved1: [u8; 3],
}

impl UpdSOS {
    pub const CLASS_ID: u8 = 9;
    pub const MESSAGE_ID: u8 = 20;
    pub const CMD_FLASH_BACKUP_CREATE: u8 = 0;
    pub const CMD_FLASH_BACKUP_CLEAR: u8 = 1;
}

impl Default for UpdSOS {
    fn default() -> Self {
        UpdSOS {
            cmd: 0,
            reserved1: [0; 3],
        }
    }
}

impl ros2_client::Message for UpdSOS {}
