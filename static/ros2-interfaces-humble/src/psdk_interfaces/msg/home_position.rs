use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomePosition {
    pub header: crate::std_msgs::msg::Header,
    pub home_position_status: u8,
    pub latitude: f64,
    pub longitude: f64,
}

impl HomePosition {
    pub const HOME_POSITION_STATUS_FAILED: u8 = 0;
    pub const HOME_POSITION_STATUS_SUCCESS: u8 = 1;
}

impl Default for HomePosition {
    fn default() -> Self {
        HomePosition {
            header: crate::std_msgs::msg::Header::default(),
            home_position_status: 0,
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl ros2_client::Message for HomePosition {}
