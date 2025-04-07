use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficSignInfo {
    pub header: crate::std_msgs::msg::Header,
    pub status: u8,
    pub camera_used: bool,
    pub navigation_used: bool,
    pub speed_units: u8,
    pub speed_limit: f32,
}

impl TrafficSignInfo {
    pub const STATUS_UNKNOWN: u8 = 0;
    pub const STATUS_OFF: u8 = 1;
    pub const STATUS_ACTIVE: u8 = 2;
    pub const STATUS_ERROR: u8 = 3;
    pub const UNIT_UNKNOWN: u8 = 0;
    pub const UNIT_KPH: u8 = 1;
    pub const UNIT_MPH: u8 = 2;
}

impl Default for TrafficSignInfo {
    fn default() -> Self {
        TrafficSignInfo {
            header: crate::std_msgs::msg::Header::default(),
            status: 0,
            camera_used: false,
            navigation_used: false,
            speed_units: 0,
            speed_limit: 0.0,
        }
    }
}

impl ros2_client::Message for TrafficSignInfo {}
