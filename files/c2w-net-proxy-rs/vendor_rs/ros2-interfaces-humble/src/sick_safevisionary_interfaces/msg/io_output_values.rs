use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOOutputValues {
    pub pin_5: u16,
    pub pin_6: u16,
    pub pin_7: u16,
    pub pin_8: u16,
}

impl Default for IOOutputValues {
    fn default() -> Self {
        IOOutputValues {
            pin_5: 0,
            pin_6: 0,
            pin_7: 0,
            pin_8: 0,
        }
    }
}

impl ros2_client::Message for IOOutputValues {}
