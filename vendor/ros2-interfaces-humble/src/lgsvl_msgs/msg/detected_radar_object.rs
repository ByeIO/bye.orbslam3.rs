use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedRadarObject {
    pub id: i32,
    pub sensor_aim: crate::geometry_msgs::msg::Vector3,
    pub sensor_right: crate::geometry_msgs::msg::Vector3,
    pub sensor_position: crate::geometry_msgs::msg::Point,
    pub sensor_velocity: crate::geometry_msgs::msg::Vector3,
    pub sensor_angle: f64,
    pub object_position: crate::geometry_msgs::msg::Point,
    pub object_velocity: crate::geometry_msgs::msg::Vector3,
    pub object_relative_position: crate::geometry_msgs::msg::Point,
    pub object_relative_velocity: crate::geometry_msgs::msg::Vector3,
    pub object_collider_size: crate::geometry_msgs::msg::Vector3,
    pub object_state: u8,
    pub new_detection: bool,
}

impl DetectedRadarObject {
    pub const STATE_MOVING: u8 = 0;
    pub const STATE_STATIONARY: u8 = 1;
}

impl Default for DetectedRadarObject {
    fn default() -> Self {
        DetectedRadarObject {
            id: 0,
            sensor_aim: crate::geometry_msgs::msg::Vector3::default(),
            sensor_right: crate::geometry_msgs::msg::Vector3::default(),
            sensor_position: crate::geometry_msgs::msg::Point::default(),
            sensor_velocity: crate::geometry_msgs::msg::Vector3::default(),
            sensor_angle: 0.0,
            object_position: crate::geometry_msgs::msg::Point::default(),
            object_velocity: crate::geometry_msgs::msg::Vector3::default(),
            object_relative_position: crate::geometry_msgs::msg::Point::default(),
            object_relative_velocity: crate::geometry_msgs::msg::Vector3::default(),
            object_collider_size: crate::geometry_msgs::msg::Vector3::default(),
            object_state: 0,
            new_detection: false,
        }
    }
}

impl ros2_client::Message for DetectedRadarObject {}
