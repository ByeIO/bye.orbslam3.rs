use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgImuData {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub imu_status: crate::sbg_driver::msg::SbgImuStatus,
    pub accel: crate::geometry_msgs::msg::Vector3,
    pub gyro: crate::geometry_msgs::msg::Vector3,
    pub temp: f32,
    pub delta_vel: crate::geometry_msgs::msg::Vector3,
    pub delta_angle: crate::geometry_msgs::msg::Vector3,
}

impl Default for SbgImuData {
    fn default() -> Self {
        SbgImuData {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            imu_status: crate::sbg_driver::msg::SbgImuStatus::default(),
            accel: crate::geometry_msgs::msg::Vector3::default(),
            gyro: crate::geometry_msgs::msg::Vector3::default(),
            temp: 0.0,
            delta_vel: crate::geometry_msgs::msg::Vector3::default(),
            delta_angle: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for SbgImuData {}
