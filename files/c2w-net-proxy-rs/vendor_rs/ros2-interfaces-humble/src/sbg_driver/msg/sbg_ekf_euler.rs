use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEkfEuler {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub angle: crate::geometry_msgs::msg::Vector3,
    pub accuracy: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgEkfStatus,
}

impl Default for SbgEkfEuler {
    fn default() -> Self {
        SbgEkfEuler {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            angle: crate::geometry_msgs::msg::Vector3::default(),
            accuracy: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgEkfStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgEkfEuler {}
