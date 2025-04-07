use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSatFix {
    pub header: crate::std_msgs::msg::Header,
    pub status: crate::sensor_msgs::msg::NavSatStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub position_covariance: [f64; 9],
    pub position_covariance_type: u8,
}

impl NavSatFix {
    pub const COVARIANCE_TYPE_UNKNOWN: u8 = 0;
    pub const COVARIANCE_TYPE_APPROXIMATED: u8 = 1;
    pub const COVARIANCE_TYPE_DIAGONAL_KNOWN: u8 = 2;
    pub const COVARIANCE_TYPE_KNOWN: u8 = 3;
}

impl Default for NavSatFix {
    fn default() -> Self {
        NavSatFix {
            header: crate::std_msgs::msg::Header::default(),
            status: crate::sensor_msgs::msg::NavSatStatus::default(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            position_covariance: [0.0; 9],
            position_covariance_type: 0,
        }
    }
}

impl ros2_client::Message for NavSatFix {}
