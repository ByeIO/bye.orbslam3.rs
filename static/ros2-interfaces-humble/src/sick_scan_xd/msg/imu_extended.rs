use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImuExtended {
    pub header: crate::std_msgs::msg::Header,
    pub imu: crate::sensor_msgs::msg::Imu,
    pub ticks: u32,
    pub quaternion_accuracy: f32,
    pub angular_velocity_reliability: u8,
    pub linear_acceleration_reliability: u8,
}

impl Default for ImuExtended {
    fn default() -> Self {
        ImuExtended {
            header: crate::std_msgs::msg::Header::default(),
            imu: crate::sensor_msgs::msg::Imu::default(),
            ticks: 0,
            quaternion_accuracy: 0.0,
            angular_velocity_reliability: 0,
            linear_acceleration_reliability: 0,
        }
    }
}

impl ros2_client::Message for ImuExtended {}
