use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgImuShort {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub imu_status: crate::sbg_driver::msg::SbgImuStatus,
    pub delta_velocity: crate::geometry_msgs::msg::Vector3,
    pub delta_angle: crate::geometry_msgs::msg::Vector3,
    pub temperature: i16,
}

impl Default for SbgImuShort {
    fn default() -> Self {
        SbgImuShort {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            imu_status: crate::sbg_driver::msg::SbgImuStatus::default(),
            delta_velocity: crate::geometry_msgs::msg::Vector3::default(),
            delta_angle: crate::geometry_msgs::msg::Vector3::default(),
            temperature: 0,
        }
    }
}

impl ros2_client::Message for SbgImuShort {}
