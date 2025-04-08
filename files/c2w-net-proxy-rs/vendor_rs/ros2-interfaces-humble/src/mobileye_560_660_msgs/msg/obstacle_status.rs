use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleStatus {
    pub header: crate::std_msgs::msg::Header,
    pub num_obstacles: u16,
    pub timestamp: u16,
    pub application_version: u16,
    pub active_version_number_section: u16,
    pub left_close_range_cut_in: bool,
    pub right_close_range_cut_in: bool,
    pub stop_go: u8,
    pub protocol_version: u8,
    pub close_car: bool,
    pub failsafe: u8,
}

impl ObstacleStatus {
    pub const STOP_GO_STOP: u8 = 0;
    pub const STOP_GO_GO: u8 = 1;
    pub const STOP_GO_UNDECIDED: u8 = 2;
    pub const STOP_GO_DRIVER_DECISION_REQUIRED: u8 = 3;
    pub const STOP_GO_NOT_CALCULATED: u8 = 15;
    pub const FAILSAFE_NONE: u8 = 0;
    pub const FAILSAFE_LOW_SUN: u8 = 1;
    pub const FAILSAFE_BLUR_IMAGE: u8 = 2;
}

impl Default for ObstacleStatus {
    fn default() -> Self {
        ObstacleStatus {
            header: crate::std_msgs::msg::Header::default(),
            num_obstacles: 0,
            timestamp: 0,
            application_version: 0,
            active_version_number_section: 0,
            left_close_range_cut_in: false,
            right_close_range_cut_in: false,
            stop_go: 0,
            protocol_version: 0,
            close_car: false,
            failsafe: 0,
        }
    }
}

impl ros2_client::Message for ObstacleStatus {}
