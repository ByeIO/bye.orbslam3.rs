use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ignition {
    pub value: u8,
}

impl Ignition {
    pub const UNKNOWN: u8 = 0;
    pub const OFF: u8 = 1;
    pub const ACCESSORY: u8 = 2;
    pub const RUN: u8 = 3;
    pub const START: u8 = 4;
}

impl Default for Ignition {
    fn default() -> Self {
        Ignition {
            value: 0,
        }
    }
}

impl ros2_client::Message for Ignition {}
