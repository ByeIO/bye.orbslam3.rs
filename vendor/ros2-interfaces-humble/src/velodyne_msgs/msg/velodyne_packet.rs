use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelodynePacket {
    pub stamp: crate::builtin_interfaces::msg::Time,
    #[serde_as(as = "[_; 1206]")]
    pub data: [u8; 1206],
}

impl Default for VelodynePacket {
    fn default() -> Self {
        VelodynePacket {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: [0; 1206],
        }
    }
}

impl ros2_client::Message for VelodynePacket {}
