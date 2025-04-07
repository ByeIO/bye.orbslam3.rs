use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOOSSDSState {
    pub ossd1a: u8,
    pub ossd1b: u8,
    pub ossd2a: u8,
    pub ossd2b: u8,
}

impl Default for IOOSSDSState {
    fn default() -> Self {
        IOOSSDSState {
            ossd1a: 0,
            ossd1b: 0,
            ossd2a: 0,
            ossd2b: 0,
        }
    }
}

impl ros2_client::Message for IOOSSDSState {}
