use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonVER {
    pub sw_version: [i8; 30],
    pub hw_version: [i8; 10],
    pub extension: Vec<crate::ublox_msgs::msg::MonVERExtension>,
}

impl MonVER {
    pub const CLASS_ID: u8 = 10;
    pub const MESSAGE_ID: u8 = 4;
}

impl Default for MonVER {
    fn default() -> Self {
        MonVER {
            sw_version: [0; 30],
            hw_version: [0; 10],
            extension: Vec::new(),
        }
    }
}

impl ros2_client::Message for MonVER {}
