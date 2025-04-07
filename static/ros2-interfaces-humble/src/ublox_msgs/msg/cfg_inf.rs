use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgINF {
    pub blocks: Vec<crate::ublox_msgs::msg::CfgINFBlock>,
}

impl CfgINF {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 2;
}

impl Default for CfgINF {
    fn default() -> Self {
        CfgINF {
            blocks: Vec::new(),
        }
    }
}

impl ros2_client::Message for CfgINF {}
