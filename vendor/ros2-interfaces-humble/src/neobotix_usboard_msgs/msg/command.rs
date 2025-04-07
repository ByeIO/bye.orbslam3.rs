use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub header: crate::std_msgs::msg::Header,
    pub command: u8,
    pub command_data: u64,
    pub set_num: u8,
    pub paraset_byte6: u8,
    pub paraset_byte5: u8,
    pub paraset_byte4: u8,
    pub paraset_byte3: u8,
    pub paraset_byte2: u8,
    pub paraset_byte1: u8,
    pub set_active_9to16: u8,
    pub set_active_1to8: u8,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            header: crate::std_msgs::msg::Header::default(),
            command: 0,
            command_data: 0,
            set_num: 0,
            paraset_byte6: 0,
            paraset_byte5: 0,
            paraset_byte4: 0,
            paraset_byte3: 0,
            paraset_byte2: 0,
            paraset_byte1: 0,
            set_active_9to16: 0,
            set_active_1to8: 0,
        }
    }
}

impl ros2_client::Message for Command {}
