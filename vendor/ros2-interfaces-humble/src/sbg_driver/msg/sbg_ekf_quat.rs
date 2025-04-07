use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEkfQuat {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub quaternion: crate::geometry_msgs::msg::Quaternion,
    pub accuracy: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgEkfStatus,
}

impl Default for SbgEkfQuat {
    fn default() -> Self {
        SbgEkfQuat {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            quaternion: crate::geometry_msgs::msg::Quaternion::default(),
            accuracy: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgEkfStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgEkfQuat {}
