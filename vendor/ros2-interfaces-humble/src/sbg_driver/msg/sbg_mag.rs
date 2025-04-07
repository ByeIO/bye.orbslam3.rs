use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgMag {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub mag: crate::geometry_msgs::msg::Vector3,
    pub accel: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgMagStatus,
}

impl Default for SbgMag {
    fn default() -> Self {
        SbgMag {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            mag: crate::geometry_msgs::msg::Vector3::default(),
            accel: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgMagStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgMag {}
