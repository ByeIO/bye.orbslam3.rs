use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilStateQuaternion {
    pub header: crate::std_msgs::msg::Header,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub angular_velocity: crate::geometry_msgs::msg::Vector3,
    pub linear_acceleration: crate::geometry_msgs::msg::Vector3,
    pub linear_velocity: crate::geometry_msgs::msg::Vector3,
    pub geo: crate::geographic_msgs::msg::GeoPoint,
    pub ind_airspeed: f32,
    pub true_airspeed: f32,
}

impl Default for HilStateQuaternion {
    fn default() -> Self {
        HilStateQuaternion {
            header: crate::std_msgs::msg::Header::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            angular_velocity: crate::geometry_msgs::msg::Vector3::default(),
            linear_acceleration: crate::geometry_msgs::msg::Vector3::default(),
            linear_velocity: crate::geometry_msgs::msg::Vector3::default(),
            geo: crate::geographic_msgs::msg::GeoPoint::default(),
            ind_airspeed: 0.0,
            true_airspeed: 0.0,
        }
    }
}

impl ros2_client::Message for HilStateQuaternion {}
