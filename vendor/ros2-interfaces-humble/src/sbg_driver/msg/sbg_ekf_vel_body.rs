use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEkfVelBody {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub velocity_accuracy: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgEkfStatus,
}

impl Default for SbgEkfVelBody {
    fn default() -> Self {
        SbgEkfVelBody {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            velocity_accuracy: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgEkfStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgEkfVelBody {}
