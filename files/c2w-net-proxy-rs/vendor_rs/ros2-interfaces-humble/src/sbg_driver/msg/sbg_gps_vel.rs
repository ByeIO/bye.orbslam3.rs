use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsVel {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: crate::sbg_driver::msg::SbgGpsVelStatus,
    pub gps_tow: u32,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub velocity_accuracy: crate::geometry_msgs::msg::Vector3,
    pub course: f32,
    pub course_acc: f32,
}

impl Default for SbgGpsVel {
    fn default() -> Self {
        SbgGpsVel {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: crate::sbg_driver::msg::SbgGpsVelStatus::default(),
            gps_tow: 0,
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            velocity_accuracy: crate::geometry_msgs::msg::Vector3::default(),
            course: 0.0,
            course_acc: 0.0,
        }
    }
}

impl ros2_client::Message for SbgGpsVel {}
