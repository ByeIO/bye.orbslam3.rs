use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: u8,
}

impl GearCommand {
    pub const NONE: u8 = 0;
    pub const NEUTRAL: u8 = 1;
    pub const DRIVE: u8 = 2;
    pub const DRIVE_2: u8 = 3;
    pub const DRIVE_3: u8 = 4;
    pub const DRIVE_4: u8 = 5;
    pub const DRIVE_5: u8 = 6;
    pub const DRIVE_6: u8 = 7;
    pub const DRIVE_7: u8 = 8;
    pub const DRIVE_8: u8 = 9;
    pub const DRIVE_9: u8 = 10;
    pub const DRIVE_10: u8 = 11;
    pub const DRIVE_11: u8 = 12;
    pub const DRIVE_12: u8 = 13;
    pub const DRIVE_13: u8 = 14;
    pub const DRIVE_14: u8 = 15;
    pub const DRIVE_15: u8 = 16;
    pub const DRIVE_16: u8 = 17;
    pub const DRIVE_17: u8 = 18;
    pub const DRIVE_18: u8 = 19;
    pub const REVERSE: u8 = 20;
    pub const REVERSE_2: u8 = 21;
    pub const PARK: u8 = 22;
    pub const LOW: u8 = 23;
    pub const LOW_2: u8 = 24;
}

impl Default for GearCommand {
    fn default() -> Self {
        GearCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: 0,
        }
    }
}

impl ros2_client::Message for GearCommand {}
