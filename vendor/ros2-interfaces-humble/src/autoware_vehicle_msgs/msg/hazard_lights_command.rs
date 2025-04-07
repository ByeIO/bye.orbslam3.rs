use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardLightsCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: u8,
}

impl HazardLightsCommand {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}

impl Default for HazardLightsCommand {
    fn default() -> Self {
        HazardLightsCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: 0,
        }
    }
}

impl ros2_client::Message for HazardLightsCommand {}
