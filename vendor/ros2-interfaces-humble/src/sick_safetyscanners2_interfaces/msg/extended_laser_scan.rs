use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLaserScan {
    pub laser_scan: crate::sensor_msgs::msg::LaserScan,
    pub reflektor_status: Vec<bool>,
    pub reflektor_median: Vec<bool>,
    pub intrusion: Vec<bool>,
}

impl Default for ExtendedLaserScan {
    fn default() -> Self {
        ExtendedLaserScan {
            laser_scan: crate::sensor_msgs::msg::LaserScan::default(),
            reflektor_status: Vec::new(),
            reflektor_median: Vec::new(),
            intrusion: Vec::new(),
        }
    }
}

impl ros2_client::Message for ExtendedLaserScan {}
