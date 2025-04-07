use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Imu {
    pub header: crate::std_msgs::msg::Header,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub orientation_covariance: [f64; 9],
    pub angular_velocity: crate::geometry_msgs::msg::Vector3,
    pub angular_velocity_covariance: [f64; 9],
    pub linear_acceleration: crate::geometry_msgs::msg::Vector3,
    pub linear_acceleration_covariance: [f64; 9],
}

impl Default for Imu {
    fn default() -> Self {
        Imu {
            header: crate::std_msgs::msg::Header::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            orientation_covariance: [0.0; 9],
            angular_velocity: crate::geometry_msgs::msg::Vector3::default(),
            angular_velocity_covariance: [0.0; 9],
            linear_acceleration: crate::geometry_msgs::msg::Vector3::default(),
            linear_acceleration_covariance: [0.0; 9],
        }
    }
}

impl ros2_client::Message for Imu {}
