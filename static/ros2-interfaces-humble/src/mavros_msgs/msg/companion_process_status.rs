use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanionProcessStatus {
    pub header: crate::std_msgs::msg::Header,
    pub state: u8,
    pub component: u8,
}

impl CompanionProcessStatus {
    pub const MAV_STATE_UNINIT: u8 = 0;
    pub const MAV_STATE_BOOT: u8 = 1;
    pub const MAV_STATE_CALIBRATING: u8 = 2;
    pub const MAV_STATE_STANDBY: u8 = 3;
    pub const MAV_STATE_ACTIVE: u8 = 4;
    pub const MAV_STATE_CRITICAL: u8 = 5;
    pub const MAV_STATE_EMERGENCY: u8 = 6;
    pub const MAV_STATE_POWEROFF: u8 = 7;
    pub const MAV_STATE_FLIGHT_TERMINATION: u8 = 8;
    pub const MAV_COMP_ID_OBSTACLE_AVOIDANCE: u8 = 196;
    pub const MAV_COMP_ID_VISUAL_INERTIAL_ODOMETRY: u8 = 197;
}

impl Default for CompanionProcessStatus {
    fn default() -> Self {
        CompanionProcessStatus {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
            component: 0,
        }
    }
}

impl ros2_client::Message for CompanionProcessStatus {}
