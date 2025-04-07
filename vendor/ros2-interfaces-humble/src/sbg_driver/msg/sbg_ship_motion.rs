use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgShipMotion {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub heave_period: u16,
    pub ship_motion: crate::geometry_msgs::msg::Vector3,
    pub acceleration: crate::geometry_msgs::msg::Vector3,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgShipMotionStatus,
}

impl Default for SbgShipMotion {
    fn default() -> Self {
        SbgShipMotion {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            heave_period: 0,
            ship_motion: crate::geometry_msgs::msg::Vector3::default(),
            acceleration: crate::geometry_msgs::msg::Vector3::default(),
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgShipMotionStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgShipMotion {}
